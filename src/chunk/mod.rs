//! Handles raw data from the database

use hyper;
use hash::{Hash, BYTE_LEN};
use std::cell::Cell;
use byteorder::{NetworkEndian, ByteOrder};

/// A chunk of raw bytes from the database
#[derive(Debug)]
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

    pub fn extract_u32(&self) -> u32 {
        let offset = self.offset.get();
        let n = NetworkEndian::read_u32(&self.chunk.0[offset..offset + 4]);
        self.offset.set(offset + 4);
        n
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
