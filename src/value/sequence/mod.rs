mod map;
mod set;
mod list;

pub use self::map::NomsMap;
pub(crate) use self::map::Map;

pub use self::set::NomsSet;
pub(crate) use self::set::Set;

pub use self::list::NomsList;
pub(crate) use self::list::List;

use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, Collection};

use hash::Hash;
use std::cmp::Ordering;

// Somethingsomething prolly tree node. See the noms source for more (meta_sequence.go).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct MetaTuple<'a> {
    pub reference: Ref<'a>,
    pub key: OrderedKey<'a>,
    pub num_leaves: u64,
}

impl<'a> Ord for MetaTuple<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.key.cmp(&other.key) }
}
impl<'a> PartialOrd<Self> for MetaTuple<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Somethingsomething key in prolly tree level. See noms source again (still meta_sequence.go)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OrderedKey<'a> {
    ByValue(Value<'a>),
    ByHash(Hash),
}
impl<'a> Ord for OrderedKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::OrderedKey::*;
        match (self, other) {
            (&ByValue(_), &ByHash(_)) => Ordering::Less,
            (&ByHash(_), &ByValue(_)) => Ordering::Greater,
            (&ByValue(ref v1), &ByValue(ref v2)) => v1.cmp(v2),
            (&ByHash(ref h1), &ByHash(ref h2)) => h1.cmp(h2),
        }
    }
}
impl<'a> PartialOrd<Self> for OrderedKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> OrderedKey<'a> {
    pub fn by_value(value: Value<'a>) -> Self {
        OrderedKey::ByValue(value)
    }

    pub fn by_hash(hash: Hash) -> Self {
        OrderedKey::ByHash(hash)
    }

    pub fn is_ordered_by_value(&self) -> bool {
        match self {
            &OrderedKey::ByValue(_) => true,
            _ => false,
        }
    }
}
