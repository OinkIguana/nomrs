//! Generic representation of a value in the database

mod conversion;
mod reference;
mod commit;
mod sequence;
mod kind;
mod structure;

pub use self::kind::Type;
pub use self::reference::Ref;
pub use self::conversion::{IntoNoms, FromNoms};
pub use self::commit::Commit;
pub use self::sequence::{NomsMap, NomsSet, NomsList};
pub use self::structure::{NomsStruct, Empty};

pub(crate) use self::sequence::{Sequence, MetaTuple, OrderedKey, Map, Set, List};
pub(crate) use self::kind::Kind;
pub(crate) use self::structure::Struct;

use chunk::Chunk;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NomsValue(Value);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum Value {
    Boolean(bool),
    Number(u64, u64),
    String(String),
    Blob(Vec<u8>),
    Value(Vec<u8>), // TODO: is this just unknown value representation?
    List(List),
    Map(Map),
    Ref(Ref),
    Set(Set),
    Struct(Struct),
    Type(Type),
    Union(Chunk),
    Nil,
}

impl NomsValue {
    pub(crate) fn import(self) -> Value {
        self.0
    }
}

impl Value {
    pub fn new(data: Vec<u8>) -> Value {
        unimplemented!();
    }

    pub fn export(self) -> NomsValue {
        NomsValue(self)
    }
}

impl Value {
    pub fn is_bool(&self) -> bool {
        match self {
            &Value::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            &Value::Number(_, _) => true,
            _ => false,
        }
    }
    pub fn to_u64(self) -> Option<u64> {
        match self {
            Value::Number(i, e) => Some(i * 2u64.pow(3 as u32)),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            &Value::String(_) => true,
            _ => false,
        }
    }
    pub fn to_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_type(&self) -> bool {
        match self {
            &Value::Type(_) => true,
            _ => false,
        }
    }
    pub fn to_type(self) -> Option<Type> {
        match self {
            Value::Type(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            &Value::Ref(_) => true,
            _ => false,
        }
    }
    pub fn to_ref(self) -> Option<Ref> {
        match self {
            Value::Ref(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            &Value::List(_) => true,
            _ => false,
        }
    }
    pub fn to_list<V>(self) -> Option<NomsList<V>>
    where V: FromNoms + IntoNoms {
        match self {
            Value::List(l) => Some(NomsList::from_list(l.transform())),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        match self {
            &Value::Map(_) => true,
            _ => false,
        }
    }
    pub fn to_map<K, V>(self) -> Option<NomsMap<K, V>>
    where K: FromNoms + IntoNoms + Eq + ::std::hash::Hash, V: FromNoms + IntoNoms {
        match self {
            Value::Map(m) => Some(NomsMap::from_map(m.transform())),
            _ => None,
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
            &Value::Set(_) => true,
            _ => false,
        }
    }
    pub fn to_set<V>(self) -> Option<NomsSet<V>>
    where V: FromNoms + IntoNoms + Eq + ::std::hash::Hash{
        match self {
            Value::Set(s) => Some(NomsSet::from_set(s.transform())),
            _ => None,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            &Value::Struct(_) => true,
            _ => false,
        }
    }
    pub fn to_struct<T: NomsStruct>(self) -> Option<T> {
        match self {
            Value::Struct(Struct{ props, .. }) => T::from_prop_list(props),
            _ => None,
        }
    }
}

impl IntoNoms for NomsValue {
    fn into_noms(&self) -> Vec<u8> {
        self.0.into_noms()
    }
}
impl FromNoms for NomsValue {
    fn from_noms(v: &Vec<u8>) -> Self {
        NomsValue(Value::from_noms(v))
    }
}
