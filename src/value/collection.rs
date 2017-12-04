//! An interface for extracting the ChunkStore from a value type, or using that ChunkStore to
//! resolve a reference.

use database::ChunkStore;
use super::{FromNoms, MetaTuple};
use error::Error;

pub(crate) trait Collection<'a, V: FromNoms<'a>> {
    fn database(&self) -> &'a ChunkStore;
    fn resolve(&self, h: &MetaTuple<'a>) -> Result<V, Error> {
        self.database()
            .get(h.reference.hash())
            .map(|v| v.export().transform())
    }
    fn resolve_all(&self, h: &Vec<MetaTuple<'a>>) -> Result<Vec<V>, Error> {
        self.database()
            .get_many(
                h   .iter()
                    .map(|t| t.reference.hash())
                    .collect()
            )
            .map(|mut m|
                h   .into_iter()
                    .map(move |mt| m.remove(&mt.reference.hash()).unwrap().export().transform())
                    .collect()
            )
    }
}
