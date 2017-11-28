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

// Somethingsomething prolly tree node. See the noms source for more (meta_sequence.go).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MetaTuple<'a> {
    pub reference: Ref<'a>,
    pub key: OrderedKey<'a>,
    pub num_leaves: u64,
}

// Somethingsomething key in prolly tree level. See noms source again (still meta_sequence.go)
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OrderedKey<'a> {
    ByValue(Value<'a>),
    ByHash(Hash),
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
