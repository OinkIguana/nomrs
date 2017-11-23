mod map;
mod set;
mod list;

pub use self::map::NomsMap;
pub(crate) use self::map::Map;

pub use self::set::NomsSet;
pub(crate) use self::set::Set;

pub use self::list::NomsList;
pub(crate) use self::list::List;

use super::{NomsValue, Value, Ref, FromNoms, IntoNoms};

use hash::Hash;
use either::Either;

// TODO: this is probably a dumb trait, so just get rid of it or make it useful
pub(crate) trait Sequence<V> {
    fn from_either(Either<Vec<V>, Vec<MetaTuple>>) -> Self;
    fn resolve(&self, Ref) {
        unimplemented!();
    }
}

// Somethingsomething prolly tree node. See the noms source for more (meta_sequence.go).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MetaTuple {
    pub reference: Ref,
    pub key: OrderedKey,
    pub num_leaves: u64,
}

// Somethingsomething key in prolly tree level. See noms source again (still meta_sequence.go)
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OrderedKey {
    pub is_ordered_by_value: bool,
    pub value: Option<Value>,
    pub hash: Option<Hash>,
}

impl OrderedKey {
    pub fn by_value(value: Value) -> Self {
        OrderedKey {
            is_ordered_by_value: true,
            value: Some(value),
            hash: None,
        }
    }

    pub fn by_hash(hash: Hash) -> Self {
        OrderedKey {
            is_ordered_by_value: false,
            value: None,
            hash: Some(hash),
        }
    }
}
