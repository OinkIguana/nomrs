use super::Chunk;
use hash::{Hash, BYTE_LEN};
use value::{Value, Kind, Ref, FromNoms};
use std::mem::transmute;
use std::collections::{HashMap, HashSet};
use byteorder::{NetworkEndian, ByteOrder};
use std::cell::Cell;
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

    pub fn extract_u16(&self) -> u16 {
        let offset = self.offset.get();
        let n = NetworkEndian::read_u16(&self.chunk.0[offset..offset + 8]);
        self.offset.set(offset + 2);
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
        let len = self.extract_u8();
        let name = String::from_utf8(self.extract_raw(len as usize).into_data()).unwrap();
        let prop_count = self.extract_u8() as usize;
        let mut props = HashMap::with_capacity(prop_count);
        for _ in 0..prop_count {
            let key = self.extract_raw_string();
            let value = self.extract_chunk();
            props.insert(key, value);
        }
        (name, props)
    }

    fn extract_raw_string(&self) -> String {
        let len = self.extract_u8();
        let offset = self.offset.get();
        let string = String::from_utf8(self.chunk.0[offset..offset + len as usize].to_vec()).unwrap();
        self.offset.set(offset + len as usize);
        string
    }

    pub fn extract_string(&self) -> String {
        assert_eq!(Kind::String, self.extract_kind());
        self.extract_raw_string()
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
            // TODO: no idea what "value" means when we get it
            Kind::Value => { Chunk::new(vec![0; 2]) }
            v => unimplemented!(
                "Reader for {:?} not yet implemented\nChunk starts with: {:?}",
                v, self.chunk.0[offset..min(offset + 21, self.chunk.0.len())].to_vec(),
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

    pub fn extract_map<K: FromNoms + Eq + ::std::hash::Hash, V: FromNoms>(&self) -> HashMap<K, V> {
        assert_eq!(Kind::Map, self.extract_kind());
        let mut map = HashMap::new();
        let entries = self.extract_u16();
        for _ in 0..entries {
            let key = self.extract_chunk();
            let value = self.extract_chunk();
            map.insert(K::from_noms(&key.into_value()), V::from_noms(&value.into_value()));
        }
        map
    }

    pub fn extract_set<V: FromNoms + ::std::hash::Hash + Eq>(&self) -> HashSet<V> {
        assert_eq!(Kind::Set, self.extract_kind());
        let len = self.extract_u16();
        let mut set = HashSet::with_capacity(len as usize);
        for _ in 0..len {
            set.insert(V::from_noms(&self.extract_chunk().into_value()));
        }
        set
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
