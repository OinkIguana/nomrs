/// Defines a database that is backed by an HTTP Noms database

use value::Value;

#[derive(Default)]
pub struct Database {
    database: String,
    dataset: String,
}

impl Database {
    pub fn new(database: String) -> Self {
        Self{ database, ..Self::default() }
    }
}

impl super::Database for Database {
    fn datasets(&self) -> Value { unimplemented!() }
}
