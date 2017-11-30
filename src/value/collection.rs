//! An interface for extracting the ValueAccess from a value type, or using that ValueAccess to
//! resolve a reference.

use std::collections::HashMap;
use database::ValueAccess;
use super::{FromNoms, MetaTuple};
use error::Error;

pub(crate) trait Collection<'a, V: FromNoms<'a>> {
    fn database(&self) -> &'a ValueAccess;
    fn resolve(&self, h: &MetaTuple<'a>) -> Result<V, Error> {
        self.database()
            .get_value(h.reference.hash())
            .map(|v| v.export().transform())
    }
    fn resolve_all(&self, h: &Vec<MetaTuple<'a>>) -> Result<HashMap<MetaTuple<'a>, V>, Error> {
        self.database()
            .get_values(
                h.iter()
                    .map(|t| t.reference.hash())
                    .collect()
            )
            .map(|m|
                m.into_iter()
                    .map(|(k, v)| (h.iter().find(|mt| mt.reference.hash() == k).clone().unwrap().clone(), v.export().transform()))
                    .collect()
            )
    }
}
