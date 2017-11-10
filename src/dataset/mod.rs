use database::Database;

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
}
