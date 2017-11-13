//! Handles raw data from the database

mod reader;
mod writer;

use hyper;
use value::Value;

pub(crate) use self::reader::ChunkReader;
pub(crate) use self::writer::ChunkWriter;

/// A chunk of raw bytes from the database
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Chunk(Vec<u8>);

impl Chunk {
    pub fn new(data: Vec<u8>) -> Self {
        Chunk(data)
    }
    pub fn from_hyper(hyper: hyper::Chunk) -> Self {
        Chunk(hyper.to_vec())
    }
    pub fn reader(&self) -> ChunkReader {
        ChunkReader::new(self)
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
        ChunkWriter::new()
    }
}
