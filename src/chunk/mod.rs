//! Handles reading and writing of data from the database

mod reader;

pub(crate) use self::reader::ChunkReader;

use database::ValueAccess;

/// A chunk of raw data, associated with a database.
#[derive(Clone, Debug)]
pub struct Chunk<'a> {
    database: Option<&'a ValueAccess>,
    data: Vec<u8>
}

impl<'a> PartialEq for Chunk<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<'a> Eq for Chunk<'a> {}

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
