//! Handles raw data from the database

use super::hash::Hash;

/// A chunk of raw data from the database
pub struct Chunk {
    hash: Hash,
    data: Vec<u8>,
}
