use super::Chunk;
use hash::{Hash, BYTE_LEN};
use value::{Value, Kind, Ref, FromNoms, IntoNoms, Map, Set, List, Sequence, MetaTuple, OrderedKey};
use std::mem::transmute;
use std::collections::HashMap;
use byteorder::{NetworkEndian, ByteOrder};
use std::cell::Cell;
use either::Either;
use std::cmp::min;

pub(crate) struct ChunkReader<'a> {
    chunk: &'a Chunk,
    offset: Cell<usize>,
}

impl<'a> ChunkReader<'a> {
    pub(super) fn new(chunk: &'a Chunk) -> Self {
        ChunkReader {
            chunk,
            offset: Cell::new(0),
        }
    }

    pub fn extract_hash(&self) -> Hash {
        let mut bytes = [0; BYTE_LEN];
        let offset = self.offset.get();
        bytes.copy_from_slice(&self.chunk.0[offset..offset + BYTE_LEN]);
        self.offset.set(offset + BYTE_LEN);
        Hash::new(bytes)
    }

    pub fn extract_u8(&self) -> u8 {
        let offset = self.offset.get();
        let n = self.chunk.0[offset];
        self.offset.set(offset + 1);
        n
    }

    pub fn extract_u32(&self) -> u32 {
        let offset = self.offset.get();
        let n = NetworkEndian::read_u32(&self.chunk.0[offset..offset + 4]);
        self.offset.set(offset + 4);
        n
    }

    pub fn extract_struct(&self) -> (String, HashMap<String, Chunk>) {
        assert_eq!(Kind::Struct, self.extract_kind());
        let name = self.read_string();
        let prop_count = self.extract_u8() as usize;
        let mut props = HashMap::with_capacity(prop_count);
        for _ in 0..prop_count {
            let key = self.read_string();
            let value = self.extract_chunk();
            props.insert(key, value);
        }
        (name, props)
    }

    fn read_string(&self) -> String {
        let len = self.extract_u8();
        let offset = self.offset.get();
        let string = String::from_utf8(self.chunk.0[offset..offset + len as usize].to_vec()).unwrap();
        self.offset.set(offset + len as usize);
        string
    }

    pub fn extract_string(&self) -> String {
        assert_eq!(Kind::String, self.extract_kind());
        self.read_string()
    }

    pub fn extract_chunk(&self) -> Chunk {
        let offset = self.offset.get();
        let kind = self.extract_kind();
        let chunk = match kind {
            Kind::Ref => Chunk::new(self.chunk.0[offset..self.offset.get() + BYTE_LEN].to_vec()),
            Kind::String => {
                let len = self.extract_u8();
                Chunk::new(self.chunk.0[offset..self.offset.get() + len as usize].to_vec())
            }
            Kind::Struct => {
                self.offset.set(offset);
                self.extract_struct();
                Chunk::new(self.chunk.0[offset..self.offset.get()].to_vec())
            }
            Kind::Set => {
                self.offset.set(offset);
                self.extract_set::<Value>();
                Chunk::new(self.chunk.0[offset..self.offset.get()].to_vec())
            }
            Kind::Map => {
                self.offset.set(offset);
                self.extract_map::<Value, Value>();
                Chunk::new(self.chunk.0[offset..self.offset.get()].to_vec())
            }
            Kind::List => {
                self.offset.set(offset);
                self.extract_list::<Value>();
                Chunk::new(self.chunk.0[offset..self.offset.get()].to_vec())
            }
            // TODO: no idea what "value" means when we get it
            //       maybe it's not supposed to actually come alone
            Kind::Value => { Chunk::new(vec![0; 2]) }
            v => unimplemented!(
                "Reader for {:?} not yet implemented\nChunk starts with: {:?}",
                v,
                // self.chunk.0[offset..min(offset + 21, self.chunk.0.len())].to_vec(),
                self.chunk.0[offset..].to_vec(),
            ),
        };
        self.offset.set(offset + chunk.0.len());
        chunk
    }

    pub fn extract_ref(&self) -> Ref {
        assert_eq!(Kind::Ref, self.extract_kind());
        Ref::new(self.extract_hash())
    }

    pub fn extract_kind(&self) -> Kind {
        unsafe{ transmute(self.extract_u8()) }
    }

    fn extract_sequence<T, F: Fn(&Self) -> T>(&self, extract: F) -> Either<Vec<T>, Vec<MetaTuple>> {
        let level = self.extract_u8();
        let len = self.extract_u8();
        let mut seq = if level == 0 {
            Either::Left(Vec::<T>::with_capacity(len as usize))
        } else {
            Either::Right(Vec::<MetaTuple>::with_capacity(len as usize))
        };
        for _ in 0..len {
            match seq.as_mut() {
                Either::Left(v) => v.push(extract(self)),
                Either::Right(v) => {
                    v.push(self.extract_metatuple());
                }
            }
        }
        seq
    }

    fn extract_metatuple(&self) -> MetaTuple {
        println!("{:?}", self.chunk.0[self.offset.get()..].to_vec());
        let reference = self.extract_ref();
        let value = self.extract_chunk();
        // then: v + numleaves
        // idk how tho...
        MetaTuple {
            reference,
            key: OrderedKey {
                is_ordered_by_value: false,
                value: Chunk::new(vec![]).into_value(),
                hash: Hash::new([0; 20]),
            },
            hash: Hash::new([0; 20]),
        }
    }

    pub fn extract_map<K: IntoNoms + FromNoms + Eq + ::std::hash::Hash, V: IntoNoms + FromNoms>(&self) -> Map<K, V> {
        assert_eq!(Kind::Map, self.extract_kind());
        Map::from_either(self.extract_sequence(|cr| ( K::from_noms(&cr.extract_chunk().into_value()), V::from_noms(&cr.extract_chunk().into_value()))))
    }

    pub fn extract_set<V: IntoNoms + FromNoms + ::std::hash::Hash + Eq>(&self) -> Set<V> {
        assert_eq!(Kind::Set, self.extract_kind());
        Set::from_either(self.extract_sequence(|cr| V::from_noms(&cr.extract_chunk().into_value())))
    }

    pub fn extract_list<V: IntoNoms + FromNoms>(&self) -> List<V> {
        assert_eq!(Kind::List, self.extract_kind());
        List::from_either(self.extract_sequence(|cr| V::from_noms(&cr.extract_chunk().into_value())))
    }

    pub fn extract_raw(&self, len: usize) -> Chunk {
        let offset = self.offset.get();
        let value = self.chunk.0[offset..offset + len].to_vec();
        self.offset.set(offset + len);
        Chunk(value)
    }

    pub fn empty(&self) -> bool {
        self.offset.get() >= self.chunk.0.len()
    }
}
