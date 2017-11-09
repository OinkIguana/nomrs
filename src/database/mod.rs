//! Manages connections to a database

mod http;

use dataset::Dataset;
use value::{Value, Ref};
use error::Error;

/// The protocol to use to connect to the database
#[derive(Clone, Copy)]
pub enum Protocol {
    Http,
    Https,
}
impl Default for Protocol {
    fn default() -> Self { Protocol::Http }
}

/// A connection to a database
#[derive(Default)]
pub struct DatabaseBuilder {
    protocol: Protocol,
    database: String,
}

pub struct CommitOptions {
    // TODO: un-generalize this when Rust<->Noms conversions are implemented
    parents: Value,
    meta: Value,
}

pub trait Database {
    /// Returns the root of the database, which is a Map<String, Ref<Commit>>, where the key is the
    /// ID of the dataset.
    fn datasets(&self) -> Value;
    fn dataset(&self, ds: String) -> Dataset;
    fn rebase(&self);
    fn commit(&self, ds: Dataset, v: Value, o: CommitOptions) -> Result<Dataset, Error>;
    fn commit_value(&self, ds: Dataset, v: Value) -> Result<Dataset, Error>;
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error>;
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;
    // TODO: implement stats at another time
    fn stats(&self) {}
    fn stats_summary(&self) -> String { "Unsupported".to_string() }
}

impl DatabaseBuilder {
    /// Creates a new connection to an HTTP database
    pub fn http(database: String) -> Self {
        Self{ protocol: Protocol::Http, database, ..Self::default() }
    }
    /// Creates a new connection to an HTTPS database
    pub fn https(database: String) -> Self {
        Self{ protocol: Protocol::Https, database, ..Self::default() }
    }

    pub fn build(self) -> Box<Database> {
        match self.protocol {
            Protocol::Http => Box::new(http::Database::new(self.database)),
            Protocol::Https => unimplemented!(),
        }
    }
}
