/// Defines a database that is backed by a Noms HTTP database

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use super::CommitOptions;
use value::{Value, Ref, IntoNoms, FromNoms};
use chunk::Chunk;
use dataset::Dataset;
use error::Error;
use http::Client;
use InnerNoms;

#[derive(Clone)]
pub struct Database {
    database: String,
    version: String,
    client: Client,
    root: Ref,
    noms: Rc<RefCell<InnerNoms>>,
    cache: RefCell<HashMap<Ref, Chunk>>,
}

impl Database {
    pub(crate) fn new(noms: Rc<RefCell<InnerNoms>>, database: String, version: String) -> Result<Self, Error> {
        let client = Client::new(database.clone(), version.clone(), &noms.borrow().event_loop.handle());
        let get_root = client.get_root();
        let root = noms.borrow_mut().event_loop.run(get_root)?;
        Ok(Self{ database, version, client, root, noms: noms.clone(), cache: RefCell::new(HashMap::new()) })
    }
}

impl Database {
    fn add_to_cache<I: IntoNoms>(&self, r: Ref, v: I) -> I {
        self.cache.borrow_mut().insert(r, v.into_noms().0);
        println!("{:?}", self.cache.borrow());
        v
    }
}

impl super::Database for Database {
    fn datasets(&self) -> Result<HashMap<String, Ref>, Error> {
        if self.root.is_empty() {
            Ok(HashMap::new())
        } else {
            self.noms
                .borrow_mut()
                .event_loop
                .run(self.client.post_get_refs(&self.root, vec![self.root.clone()]))
                .map(|mut v| HashMap::from_noms(&Value(v.remove(&self.root).unwrap())))
                .map(|v| self.add_to_cache(self.root.clone(), v))
        }
    }
    fn dataset<'a>(&'a self, ds: String) -> Dataset<'a> {
        Dataset::new(self, ds)
    }
    fn rebase(&self) { unimplemented!() }
    fn commit(&self, ds: Dataset, v: Value, o: CommitOptions) -> Result<Dataset, Error> { unimplemented!() }
    fn commit_value(&self, ds: Dataset, v: Value) -> Result<Dataset, Error> { unimplemented!() }
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error> { unimplemented!() }
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
}
