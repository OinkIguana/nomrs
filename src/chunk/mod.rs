//! Handles raw data from the database

use hyper;
use hash::{Hash, BYTE_LEN};
use std::cell::Cell;
use byteorder::{NetworkEndian, ByteOrder};
use value::{Kind, Ref};
use std::mem::transmute;

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

    pub fn extract_string(&self) -> String {
        assert_eq!(Kind::String, self.extract_kind());
        let len = self.extract_u8();
        let offset = self.offset.get();
        let string = String::from_utf8(self.chunk.0[offset..offset + len as usize].to_vec()).unwrap();
        self.offset.set(offset + len as usize);
        string
    }

    pub fn extract_chunk(&self) -> Chunk {
        let offset = self.offset.get();
        let kind = self.extract_kind();
        let chunk = match kind {
            Kind::Ref => Chunk::new(self.chunk.0[offset..self.offset.get() + BYTE_LEN].to_vec()),
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
