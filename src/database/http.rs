//! Defines a database that is backed by a Noms HTTP database

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use super::{CommitOptions, ValueAccess};
use value::{NomsValue, Value, Ref, FromNoms, IntoNoms, NomsMap};
use dataset::Dataset;
use error::Error;
use http::Client;
use hash::Hash;
use InnerNoms;
use chunk::{Chunk};

#[derive(Clone)]
pub struct Database {
    database: String,
    version: String,
    client: Client,
    root: Hash,
    noms: Rc<RefCell<InnerNoms>>,
    cache: RefCell<HashMap<Hash, Vec<u8>>>,
}
impl ::std::fmt::Debug for Database {
    fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) }
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
    fn add_to_cache<I: IntoNoms>(&self, h: Hash, v: I) -> I {
        self.cache.borrow_mut().insert(h, v.into_noms());
        v
    }
}

impl super::Database for Database {
    fn datasets(&self) -> Result<NomsMap<String, Ref>, Error> {
        if self.root.is_empty() {
            Ok(NomsMap::new(self))
        } else {
            self.get_value(self.root)
                .and_then(|v| v.to_map().ok_or(Error::ConversionError("Value is not a map".to_string())))
        }
    }
    fn dataset<'a>(&'a self, ds: &str) -> Result<Dataset<'a>, Error> {
        let r = self.datasets()?
            .get(ds)
            .ok_or_else(|| Error::NoDataset(ds.to_string()))?
            .clone();
        Ok(Dataset::new(self, ds, r))
    }
    fn rebase(&self) { unimplemented!() }
    fn commit(&self, ds: Dataset, v: NomsValue, o: CommitOptions) -> Result<Dataset, Error> { unimplemented!() }
    fn commit_value(&self, ds: Dataset, v: NomsValue) -> Result<Dataset, Error> { unimplemented!() }
    fn delete(&self, ds: Dataset) -> Result<Dataset, Error> { unimplemented!() }
    fn set_head(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
    fn fast_forward(&self, ds: Dataset, head: Ref) -> Result<Dataset, Error> { unimplemented!() }
}

impl super::ValueAccess for Database {
    fn get_value(&self, r: Hash) -> Result<Value, Error> {
        let cached = self.cache.borrow().get(&r).cloned();
        match cached {
            Some(data) => Ok(Value::from_noms(&Chunk::new(self, data))),
            None =>
                self.noms.borrow_mut()
                    .event_loop
                    .run(self.client.post_get_refs(self, vec![r]))
                    .and_then(|mut v| v.remove(&r).ok_or(Error::NoValueForRef(r)))
                    .map(|v| self.add_to_cache(r.clone(), v))
                    .map(|c| Chunk::new(self, c).reader().read_raw_value())
        }
    }
}
