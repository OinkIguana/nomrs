/// Defines a database that is backed by a Noms HTTP database

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use super::CommitOptions;
use value::{Value, Ref};
use dataset::Dataset;
use error::Error;
use http::Client;
use hash::Hash;
use InnerNoms;

#[derive(Clone)]
pub struct Database {
    database: String,
    version: String,
    client: Client,
    root: Hash,
    noms: Rc<RefCell<InnerNoms>>,
}

impl Database {
    pub(super) fn new(noms: Rc<RefCell<InnerNoms>>, database: String, version: String) -> Result<Self, Error> {
        let client = Client::new(database.clone(), version.clone(), &noms.borrow().event_loop.handle());
        let get_root = client.get_root();
        let root = noms.borrow_mut().event_loop.run(get_root)?;
        Ok(Self{ database, version, client, root, noms: noms.clone() })
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
