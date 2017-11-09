//! Manages connections to a database

use dataset::Dataset;

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
#[derive(Clone, Default)]
pub struct Database {
    pub protocol: Protocol,
    pub database: String,
    pub dataset: String,
}

impl Database {
    /// Creates a new connection to an HTTP database
    pub fn http(database: String) -> Self {
        Self{ protocol: Protocol::Http, database, ..Self::default() }
    }
    /// Creates a new connection to an HTTPS database
    pub fn https(database: String) -> Self {
        Self{ protocol: Protocol::Https, database, ..Self::default() }
    }
    /// Create's a handle to some dataset in the database
    pub fn dataset(&self, dataset: String) -> Dataset {
        Dataset::new(&self, dataset)
    }
    /// Closes this database. Databases are automatically closed when dropped
    pub fn close(self) {}
}
