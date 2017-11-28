//! Handles reading and writing of data from the database

mod reader;
// mod writer;

pub(crate) use self::reader::ChunkReader;
// pub(crate) use self::writer::ChunkWriter;

use database::ValueAccess;

/// A chunk of raw data, associated with a database.
#[derive(Clone, Debug)]
pub struct Chunk<'a> {
    database: Option<&'a ValueAccess>,
    data: Vec<u8>
}

impl<'a> Chunk<'a> {
    pub(crate) fn new(database: &'a ValueAccess, data: Vec<u8>) -> Self {
        Self { database: Some(database), data }
    }
    pub(crate) fn maybe(database: Option<&'a ValueAccess>, data: Vec<u8>) -> Self {
        Self { database, data }
    }

    pub(crate) fn reader(&self) -> ChunkReader<'a> {
        ChunkReader::new(self.database, &self.data)
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
