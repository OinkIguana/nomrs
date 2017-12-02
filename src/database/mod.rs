//! Manages connections to a database

mod http;

use std::cell::RefCell;
use std::rc::Rc;
use dataset::Dataset;
use value::{NomsValue, NomsStruct, Value, Ref, NomsMap, FromNoms, IntoNoms};
use error::Error;
use hash::Hash;
use InnerNoms;
use std::collections::HashMap;

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

pub struct CommitOptions<'a> {
    pub parents: NomsValue<'a>,
    pub meta: NomsValue<'a>,
}
impl<'a> Default for CommitOptions<'a> {
    fn default() -> Self {
        CommitOptions{
            parents: Value::Nil.export(),
            meta: Value::Nil.export(),
        }
    }
}

/// A trait providing full access to the underlying Noms database.
pub trait Database {

    // Noms API

    /// Returns the root of the database, which is a Map<String, Ref<Commit>>, where the key is the
    /// ID of the dataset.
    fn datasets(&self) -> Result<NomsMap<String, Ref>, Error>;
    /// Gets the Dataset corresponding to the given ds dataset ID from the datasets map.
    fn dataset<'a, M, V>(&'a self, ds: &str) -> Result<Dataset<M, V>, Error>
    where M: FromNoms<'a> + IntoNoms + NomsStruct<'a>, V: FromNoms<'a> + IntoNoms, Self: Sized;
    fn rebase(&self);
    fn commit(&self, ds: Dataset, v: NomsValue, o: CommitOptions) -> Result<Dataset, Error>;
    fn commit_value(&self, ds: Dataset, v: NomsValue) -> Result<Dataset, Error>;
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error>;
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error>;

    // TODO: implement stats at another time?
    fn stats(&self) {}
    fn stats_summary(&self) -> String { UNSUPPORTED.to_string() }

    fn value_from<'a, I>(&'a self, value: I) -> NomsValue<'a>
    where I: IntoNoms, Self: Sized;
}

// TODO: this debug thing is just for compiling during development... fix it later. It should not
//       be a requirement
pub(crate) trait ValueAccess: Database + ::std::fmt::Debug {
    fn get_value(&self, h: Hash) -> Result<Value, Error> {
        self.get_values(vec![h]).map(|mut v| v.remove(&h).unwrap())
    }
    fn get_values(&self, Vec<Hash>) -> Result<HashMap<Hash, Value>, Error>;
}

/// Used to construct a new connection to the database
pub struct DatabaseBuilder {
    version: String,
    noms: Rc<RefCell<InnerNoms>>,
}

impl DatabaseBuilder {
    pub(crate) fn new(noms: Rc<RefCell<InnerNoms>>) -> Self {
        DatabaseBuilder{ noms, version: DEFAULT_VERSION.to_string() }
    }
    /// Creates a new connection to an HTTP database
    pub fn http(self, database: &str) -> Result<http::Database, Error> {
        Ok(http::Database::new(self.noms, database.to_string(), self.version)?)
    }
    /// Creates a new connection to an HTTPS database
    pub fn https(self, database: &str) -> Result<http::Database, Error> {
        Err(Error::Unimplemented("HTTPS connections are not implemented".to_string()))
    }

    /// Sets the Noms version number, required for the request header
    pub fn noms_version(self, version: &str) -> Self {
        Self{ version: version.to_string(), ..self }
    }
}
