use super::Chunk;
use hash::Hash;
use value::{Value, Type, Kind, Ref, FromNoms, IntoNoms, Map};
use byteorder::{NetworkEndian, ByteOrder};


pub(crate) struct ChunkWriter(Vec<u8>);

// TODO: most of this is wrong, and just exists as a stub for future
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

    pub fn write_kind(self, kind: Kind) -> Self {
        self.write_u8(kind as u8)
    }

    pub fn write_type(self, t: Type) -> Self {
        unimplemented!();
        // self.write_kind(t.kind)
    }

    pub fn write_ref(self, r: &Ref) -> Self {
        self.write_kind(Kind::Ref)
            .write_hash(r.hash())
    }

    pub fn write_map<K: FromNoms + IntoNoms + Eq + ::std::hash::Hash, V: IntoNoms + FromNoms>(mut self, map: &Map<K, V>) -> Self {
        unimplemented!();
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
