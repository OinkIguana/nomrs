/// Defines a database that is backed by a Noms HTTP database

use std::collections::HashMap;
use value::{Value, Ref};
use dataset::Dataset;
use error::Error;
use http::Client;
use hash::Hash;
use super::CommitOptions;
use Noms;

#[derive(Clone)]
pub struct Database {
    database: String,
    version: String,
    client: Client,
    root: Hash,
}

impl Database {
    pub fn new(noms: &mut Noms, database: String, version: String) -> Result<Self, Error> {
        let client = Client::new(database.clone(), version.clone(), &noms.event_loop.handle());
        let get_root = client.get_root();
        let root = noms.event_loop.run(get_root)?;
        Ok(Self{ database, version, client, root })
    }
}

impl super::Database for Database {
    fn datasets(&self) -> HashMap<String, Value> { unimplemented!() }
    fn dataset(&self, ds: String) -> Dataset { unimplemented!() }
    fn rebase(&self) { unimplemented!() }
    fn commit(&self, ds: Dataset, v: Value, o: CommitOptions) -> Result<Dataset, Error> { unimplemented!() }
    fn commit_value(&self, ds: Dataset, v: Value) -> Result<Dataset, Error> { unimplemented!() }
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error> { unimplemented!() }
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
}
