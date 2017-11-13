use super::Chunk;
use hash::Hash;
use value::{Value, Kind, Ref, IntoNoms};
use byteorder::{NetworkEndian, ByteOrder};
use std::collections::HashMap;


pub(crate) struct ChunkWriter(Vec<u8>);

impl ChunkWriter {
    pub(super) fn new() -> Self {
        ChunkWriter(vec![])
    }
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

    fn write_kind(self, kind: Kind) -> Self {
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
