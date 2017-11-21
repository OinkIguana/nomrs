//! Manages connections to a database

mod http;

use std::cell::RefCell;
use std::rc::Rc;
use dataset::Dataset;
use value::{Value, Ref, NomsMap};
use error::Error;
use hash::Hash;
use InnerNoms;

const DEFAULT_VERSION: &'static str = "7.18";
const UNSUPPORTED: &'static str = "Unsupported";

/// The protocol to use to connect to the database
#[derive(Clone, Copy)]
pub enum Protocol {
    Http,
    Https,
}
impl Default for Protocol {
    fn default() -> Self { Protocol::Http }
}

pub struct CommitOptions {
    // TODO: un-generalize this when Rust<->Noms conversions are implemented?
    parents: Value,
    meta: Value,
}

/// A trait providing full access to the underlying Noms database.
pub trait Database {
    /// Returns the root of the database, which is a Map<String, Ref<Commit>>, where the key is the
    /// ID of the dataset.
    fn datasets(&self) -> Result<NomsMap<String, Ref>, Error>;
    /// Gets the Dataset corresponding to the given ds dataset ID from the datasets map.
    fn dataset(&self, ds: &str) -> Result<Dataset, Error>;
    fn rebase(&self);
    fn commit(&self, ds: Dataset, v: Value, o: CommitOptions) -> Result<Dataset, Error>;
    fn commit_value(&self, ds: Dataset, v: Value) -> Result<Dataset, Error>;
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error>;
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;
    // TODO: implement stats at another time
    fn stats(&self) {}
    fn stats_summary(&self) -> String { UNSUPPORTED.to_string() }
}

pub(crate) trait ValueAccess: Database {
    fn get_value(&self, Hash) -> Result<Value, Error>;
}

/// Used to construct a new connection to the database
pub struct DatabaseBuilder {
    protocol: Protocol,
    database: String,
    version: String,
    noms: Rc<RefCell<InnerNoms>>,
}

impl DatabaseBuilder {
    pub(crate) fn new(noms: Rc<RefCell<InnerNoms>>) -> Self {
        DatabaseBuilder{ noms, protocol: Protocol::Http, database: "".to_string(), version: DEFAULT_VERSION.to_string() }
    }
    /// Creates a new connection to an HTTP database
    pub fn http(self, database: &str) -> Self {
        Self{ protocol: Protocol::Http, database: database.to_string(), ..self }
    }
    /// Creates a new connection to an HTTPS database
    pub fn https(self, database: &str) -> Self {
        Self{ protocol: Protocol::Https, database: database.to_string(), ..self }
    }
    /// Sets the Noms version number, required for the request header
    pub fn noms_version(self, version: &str) -> Self {
        Self{ version: version.to_string(), ..self }
    }
    /// Constructs the actual database, returning any errors that may occur
    pub fn build(self) -> Result<Box<Database>, Error> {
        match self.protocol {
            Protocol::Http => Ok(Box::new(http::Database::new(self.noms, self.database, self.version)?)),
            Protocol::Https => unimplemented!(),
        }
    }
}
