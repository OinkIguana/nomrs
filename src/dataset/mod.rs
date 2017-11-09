//! Manages handles to datasets, within a database

use database::Database;

#[allow(dead_code)]
pub struct Dataset<'a> {
    database: &'a Database,
    dataset: String,
}

impl<'a> Dataset<'a> {
    pub fn new(database: &'a Database, dataset: String) -> Self {
        Self{ database, dataset }
    }
}
