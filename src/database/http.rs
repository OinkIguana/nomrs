/// Defines a database that is backed by an HTTP Noms database

use value::{Value, Ref};
use dataset::Dataset;
use error::Error;
use super::CommitOptions;

#[derive(Default)]
pub struct Database {
    database: String,
    dataset: String,
    version: String,
}

impl Database {
    pub fn new(database: String, version: String) -> Self {
        Self{ database, version, ..Self::default() }
    }
}

impl super::Database for Database {
    fn datasets(&self) -> Value { unimplemented!() }
    fn dataset(&self, ds: String) -> Dataset { unimplemented!() }
    fn rebase(&self) { unimplemented!() }
    fn commit(&self, ds: Dataset, v: Value, o: CommitOptions) -> Result<Dataset, Error> { unimplemented!() }
    fn commit_value(&self, ds: Dataset, v: Value) -> Result<Dataset, Error> { unimplemented!() }
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error> { unimplemented!() }
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
}
