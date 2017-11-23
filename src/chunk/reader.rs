use super::Chunk;
use hash::{Hash, BYTE_LEN};
use value::{Value, Type, Kind, Ref, FromNoms, IntoNoms, Map, Set, List, Sequence, MetaTuple, OrderedKey};
use std::mem::transmute;
use std::collections::HashMap;
use byteorder::{NetworkEndian, ByteOrder};
use std::cell::Cell;
use either::Either;
use std::cmp::min;

pub(crate) struct ChunkReader<'a> {
    chunk: &'a Vec<u8>,
    offset: Cell<usize>,
}

fn split_varint(i: u8) -> (bool, u64) {
    (i & 0b10000000 == 1, (i & 0b01111111) as u64)
}

impl<'a> ChunkReader<'a> {
    pub(crate) fn new(chunk: &'a Vec<u8>) -> Self {
        ChunkReader {
            chunk,
            offset: Cell::new(0),
        }
    }

    pub fn read_kind(&self) -> Kind {
        unsafe{ transmute(self.read_u8()) }
    }

    pub fn read_type(&self) -> Type {
        let kind = self.read_kind();
        if kind.is_primitive() {
            Type::primitive(kind)
        } else if kind == Kind::Struct {
            let name = self.read_utf8();
            let count = self.read_varint() as usize;
            let mut props = Vec::with_capacity(count);
            let mut types = Vec::with_capacity(count);
            let mut optional = Vec::with_capacity(count);
            for _ in 0..count {
                props.push(self.read_utf8());
            }
            for _ in 0..count {
                types.push(self.read_type());
            }
            for _ in 0..count {
                optional.push(self.read_u8() == 1);
            }
            Type::structure(name, props, types, optional)
        } else if kind == Kind::Union {
            let count = self.read_varint() as usize;
            let mut types = Vec::with_capacity(count);
            for _ in 0..count {
                types.push(self.read_type());
            }
            Type::compound(kind, types)
        } else {
            let types = vec![self.read_type()];
            Type::compound(kind, types)
        }
    }

    pub fn read_hash(&self) -> Hash {
        let mut bytes = [0; BYTE_LEN];
        let offset = self.offset.get();
        bytes.copy_from_slice(&self.chunk[offset..offset + BYTE_LEN]);
        self.offset.set(offset + BYTE_LEN);
        Hash::new(bytes)
    }

    pub fn read_u8(&self) -> u8 {
        let offset = self.offset.get();
        let n = self.chunk[offset];
        self.offset.set(offset + 1);
        n
    }

    // TODO: make this handle negative varints
    fn read_varint(&self) -> u64 {
        let (msb, bits) = split_varint(self.read_u8());
        if msb {
            bits | (self.read_varint() << 7)
        } else {
            bits
        }
    }

    pub fn read_boolean(&self) -> bool {
        assert_eq!(Kind::Boolean, self.read_kind());
        self.read_u8() == 1
    }

    // TODO: make this handle more arbitrary numbers
    pub fn read_number(&self) -> (u64, u64) {
        assert_eq!(Kind::Number, self.read_kind());
        (self.read_varint(), self.read_varint())
    }

    pub fn read_struct(&self) -> (String, HashMap<String, Chunk>) {
        assert_eq!(Kind::Struct, self.read_kind());
        let name = self.read_utf8();
        let prop_count = self.read_u8() as usize;
        let mut props = HashMap::with_capacity(prop_count);
        for _ in 0..prop_count {
            let key = self.read_utf8();
            let value = self.read_chunk();
            props.insert(key, value);
        }
        (name, props)
    }

    fn read_utf8(&self) -> String {
        let len = self.read_varint();
        let offset = self.offset.get();
        let string = String::from_utf8(self.chunk[offset..offset + len as usize].to_vec()).unwrap();
        self.offset.set(offset + len as usize);
        string
    }

    pub fn read_string(&self) -> String {
        assert_eq!(Kind::String, self.read_kind());
        self.read_utf8()
    }

    pub fn read_chunk(&self) -> Chunk {
        let offset = self.offset.get();
        let kind = self.read_kind();
        self.offset.set(offset);
        match kind {
            Kind::Ref       => { self.read_ref(); }
            Kind::Boolean   => { self.read_boolean(); }
            Kind::Number    => { self.read_number(); }
            Kind::String    => { self.read_string(); }
            Kind::Struct    => { self.read_struct(); }
            Kind::Set       => { self.read_set::<Value>(); }
            Kind::Map       => { self.read_map::<Value, Value>(); }
            Kind::List      => { self.read_list::<Value>(); }
            v => unimplemented!(
                "Reader for {:?} not yet implemented\nChunk starts with: {:?}",
                v,
                self.chunk[offset..min(offset + 21, self.chunk.len())].to_vec(),
                // self.chunk[offset..].to_vec(),
            ),
        };
        Chunk::new(self.chunk[offset..self.offset.get()].to_vec())
    }

    pub fn read_ref(&self) -> Ref {
        assert_eq!(Kind::Ref, self.read_kind());
        Ref::new(self.read_hash(), self.read_type(), self.read_varint())
    }

    fn read_sequence<T, F: Fn(&Self) -> T>(&self, extract: F) -> Either<Vec<T>, Vec<MetaTuple>> {
        let level = self.read_varint();
        let len = self.read_varint();
        let mut seq = if level == 0 {
            Either::Left(Vec::<T>::with_capacity(len as usize))
        } else {
            Either::Right(Vec::<MetaTuple>::with_capacity(len as usize))
        };
        for _ in 0..len {
            match seq.as_mut() {
                Either::Left(v) => v.push(extract(self)),
                Either::Right(v) => v.push(self.read_metatuple())
            }
        }
        seq
    }

    fn read_ordered_key(&self) -> OrderedKey {
        let offset = self.offset.get();
        let kind = self.read_kind();
        if kind == Kind::Hash {
            // TODO: make constructors for this instead of just making them
            OrderedKey {
                is_ordered_by_value: false,
                value: None,
                hash: Some(self.read_hash()),
            }
        } else {
            self.offset.set(offset);
            OrderedKey {
                is_ordered_by_value: true,
                value: Some(self.read_chunk().into_value()),
                hash: None,
            }
        }
    }

    fn read_metatuple(&self) -> MetaTuple {
        MetaTuple {
            reference: self.read_ref(),
            key: self.read_ordered_key(),
            num_leaves: self.read_varint(),
        }
    }

    pub fn read_map<K: IntoNoms + FromNoms + Eq + ::std::hash::Hash, V: IntoNoms + FromNoms>(&self) -> Map<K, V> {
        assert_eq!(Kind::Map, self.read_kind());
        Map::from_either(self.read_sequence(|cr| ( K::from_noms(&cr.read_chunk().data()), V::from_noms(&cr.read_chunk().data()))))
    }

    pub fn read_set<V: IntoNoms + FromNoms + ::std::hash::Hash + Eq>(&self) -> Set<V> {
        assert_eq!(Kind::Set, self.read_kind());
        Set::from_either(self.read_sequence(|cr| V::from_noms(&cr.read_chunk().data())))
    }

    pub fn read_list<V: IntoNoms + FromNoms>(&self) -> List<V> {
        assert_eq!(Kind::List, self.read_kind());
        List::from_either(self.read_sequence(|cr| V::from_noms(&cr.read_chunk().data())))
    }

    pub fn empty(&self) -> bool {
        self.offset.get() >= self.chunk.len()
    }

    pub fn read_u32(&self) -> u32 {
        let offset = self.offset.get();
        let n = NetworkEndian::read_u32(&self.chunk[offset..offset + 4]);
        self.offset.set(offset + 4);
        n
    }

    pub fn read_raw(&self, len: usize) -> Chunk {
        let offset = self.offset.get();
        let value = self.chunk[offset..offset + len].to_vec();
        self.offset.set(offset + len);
        Chunk(value)
    }
}
