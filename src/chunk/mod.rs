//! Handles raw data from the database

use hyper;
use hash::{Hash, BYTE_LEN};
use std::cell::Cell;
use byteorder::{NetworkEndian, ByteOrder};
use value::{Value, Kind, Ref, IntoNoms, FromNoms};
use std::mem::transmute;
use std::collections::HashMap;

/// A chunk of raw bytes from the database
#[derive(Clone, Debug)]
pub(crate) struct Chunk(Vec<u8>);

impl Chunk {
    pub fn new(data: Vec<u8>) -> Self {
        Chunk(data)
    }
    pub fn from_hyper(hyper: hyper::Chunk) -> Self {
        Chunk(hyper.to_vec())
    }
    pub fn reader(&self) -> ChunkReader {
        ChunkReader {
            chunk: self,
            offset: Cell::new(0),
        }
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }
    pub fn into_data(self) -> Vec<u8> {
        self.0
    }
    pub fn into_value(self) -> Value {
        Value(self)
    }
    pub fn writer() -> ChunkWriter {
        ChunkWriter(vec![])
    }
}

pub(crate) struct ChunkReader<'a> {
    chunk: &'a Chunk,
    offset: Cell<usize>,
}

impl<'a> ChunkReader<'a> {
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
            _ => unimplemented!(),
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
        let mut map = HashMap::new();
        assert_eq!(Kind::Map, self.extract_kind());
        self.skip(1); // idk what this byte is for yet
        let entries = self.extract_u8();
        for _ in 0..entries {
            let key = self.extract_chunk();
            let value = self.extract_chunk();
            map.insert(K::from_noms(&Value(key)), V::from_noms(&Value(value)));
        }
        map
    }

    pub fn extract_raw(&self, len: usize) -> Chunk {
        let offset = self.offset.get();
        let value = self.chunk.0[offset..offset + len].to_vec();
        self.offset.set(offset + len);
        Chunk(value)
    }

    pub fn skip(&self, skip: usize) {
        self.offset.set(self.offset.get() + skip);
    }

    pub fn empty(&self) -> bool {
        self.offset.get() >= self.chunk.0.len()
    }
}


pub(crate) struct ChunkWriter(Vec<u8>);

impl ChunkWriter {
    pub fn write_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.0.extend(bytes);
        self
    }

    pub fn write_u8(self, v: u8) -> Self {
        self.write_bytes(vec![v])
    }

    pub fn write_u16(self, v: u16) -> Self {
        let mut buf = [0; 2];
        NetworkEndian::write_u16(&mut buf, v);
        self.write_bytes(buf.to_vec())
    }

    pub fn write_u32(self, v: u32) -> Self {
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, v);
        self.write_bytes(buf.to_vec())
    }

    pub fn write_hash(self, hash: Hash) -> Self {
        self.write_bytes(hash.raw_bytes().to_vec())
    }

    pub fn write_kind(self, kind: Kind) -> Self {
        self.write_u8(kind as u8)
    }

    pub fn write_ref(self, r: &Ref) -> Self {
        self.write_kind(Kind::Ref)
            .write_hash(r.hash())
    }

    pub fn write_map<K: IntoNoms + Eq + ::std::hash::Hash, V: IntoNoms>(mut self, map: &HashMap<K, V>) -> Self {
        self = self.write_kind(Kind::Map)
            .write_u16(map.len() as u16);
        for (k, v) in map {
            self = self
                .write_value(k.into_noms())
                .write_value(v.into_noms())
        }
        self
    }

    pub fn write_value(self, value: Value) -> Self {
        self.write_bytes(value.into_raw())
    }

    pub fn write_string(self, string: &str) -> Self {
        self.write_kind(Kind::String)
            .write_u8(string.len() as u8)
            .write_bytes(string.as_bytes().to_vec())
    }

    pub fn finish(self) -> Chunk {
        Chunk::new(self.0)
    }
}
