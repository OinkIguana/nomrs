//! Handles raw data from the database

mod reader;
mod writer;

use hyper;
use value::Value;

pub(crate) use self::reader::ChunkReader;
pub(crate) use self::writer::ChunkWriter;

/// A chunk of raw bytes from the database
///
/// This is an internal representation of data yet to be converted to an actual Rust type.
/// External users of this crate will be given `Value`s, which are just wrappers around
/// these `Chunk`s.
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
        ChunkReader::new(&self.0)
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }
    pub fn into_data(self) -> Vec<u8> {
        self.0
    }
    pub fn into_value(self) -> Value {
        Value::new(self.0)
    }
    pub fn writer() -> ChunkWriter {
        ChunkWriter::new()
    }
}
