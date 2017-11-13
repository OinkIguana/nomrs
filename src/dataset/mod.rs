use database::Database;
use value::{Value, Commit, Ref};

pub struct Dataset<'a> {
    dataset: String,
    database: &'a Database,
    reference: Ref,
}

impl<'a> Dataset<'a> {
    pub fn new(database: &'a Database, dataset: String, reference: Ref) -> Self {
        Self {
            dataset,
            database,
            reference,
        }
    }

    pub fn id(&self) -> &str { &self.dataset }

    pub fn head(&self) -> Option<Commit> { unimplemented!() }
    pub fn head_value(&self) -> Option<Value> { unimplemented!() }
    pub fn head_ref(&self) -> &Ref { &self.reference }
}
