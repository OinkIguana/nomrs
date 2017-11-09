//! Manages connections to a database

mod http;

use dataset::Dataset;
use value::Value;

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

pub trait Database {
    /// Returns the root of the database, which is a Map<String, Ref<Commit>>, where the key is the
    /// ID of the dataset.
    fn datasets(&self) -> Value;
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
