use database::Database;
use value::{Value, Commit, Ref};

pub struct Dataset<'a> {
    dataset: String,
    database: &'a Database,
}

impl<'a> Dataset<'a> {
    pub fn new(database: &'a Database, dataset: String) -> Self {
        Self {
            dataset,
            database,
        }
    }

    pub fn id(&self) -> String { self.dataset.clone() }

    pub fn head(&self) -> Option<Commit> { unimplemented!() }
    pub fn head_value(&self) -> Option<Value> { unimplemented!() }
    pub fn head_ref(&self) -> Option<Ref> { unimplemented!() }
}
