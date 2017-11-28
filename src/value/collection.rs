//! An interface for extracting the ValueAccess from a value type, or using that ValueAccess to
//! resolve a reference.

use hash::Hash;
use database::ValueAccess;
use super::FromNoms;
use error::Error;

pub(crate) trait Collection<'a, V: FromNoms<'a>> {
    fn database(&self) -> &'a ValueAccess;
    fn resolve(&self, h: Hash) -> Result<V, Error> {
        self.database()
            .get_value(h)
            .map(|v| v.export().transform())
    }
}
