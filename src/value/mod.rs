//! Generic representation of a value in the database
mod conversion;
mod reference;
mod commit;
mod sequence;
mod kind;
mod structure;
mod collection;
mod varint;

pub use self::kind::Type;
pub use self::reference::Ref;
pub use self::commit::Commit;
pub use self::sequence::{NomsMap, NomsSet, NomsList};
pub use self::structure::Empty;

pub(crate) use self::sequence::{MetaTuple, OrderedKey, Map, Set, List};
pub(crate) use self::conversion::{IntoNoms, FromNoms};
pub(crate) use self::kind::Kind;
pub(crate) use self::collection::Collection;
pub(crate) use self::structure::{NomsStruct, Struct};

use chunk::Chunk;
use hash::{hash, Hash};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NomsValue<'a>(Value<'a>);

impl<'a> NomsValue<'a> {
    pub(crate) fn import(self) -> Value<'a> {
        self.0
    }

    pub fn transform<T: FromNoms<'a>>(self) -> T {
        T::from_noms(&self.import().to_chunk())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Value<'a> {
    Boolean(bool),
    Number(u64, u64),
    String(String),
    Blob(Vec<u8>),
    Value(Chunk<'a>), // TODO: is this just unknown value representation?
    List(List<'a>),
    Map(Map<'a>),
    Ref(Ref<'a>),
    Set(Set<'a>),
    Struct(Struct<'a>),
    Type(Type),
    Union(Box<Value<'a>>),
    Nil,
}

impl<'a> Ord for Value<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Value::*;
        match (self, other) {
            (&Boolean(a), &Boolean(b)) => a.cmp(&b),
            (&Number(a1, a2), &Number(b1, b2)) => (a1 as f64).powi(a2 as i32).partial_cmp(&(b1 as f64).powi(b2 as i32)).unwrap(),
            (&String(ref a), &String(ref b)) => a.cmp(b),
            (&Boolean(_), _) => Ordering::Less,
            (_, &Boolean(_)) => Ordering::Greater,
            (&Number(_, _), _) => Ordering::Less,
            (_, &Number(_, _)) => Ordering::Greater,
            (&String(_), _) => Ordering::Less,
            (_, &String(_)) => Ordering::Greater,
            (_, _) => self.compute_hash().cmp(&other.compute_hash())
        }
    }
}
impl<'a> PartialOrd<Self> for Value<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ::std::hash::Hash for Value<'a> {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        match self {
            &Value::Ref(ref r) => { r.hash().hash(state); } // TODO: is this right?
            _ => { self.compute_hash().hash(state) }
        }
    }
}

impl<'a> Value<'a> {
    pub fn new(chunk: Chunk<'a>) -> Value<'a> {
        Value::Value(chunk)
    }

    pub fn export(self) -> NomsValue<'a> {
        NomsValue(self)
    }
}

impl<'a> Value<'a> {
    pub fn compute_hash(&self) -> Hash {
        hash(&self.into_noms())
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Value::Boolean(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Boolean,
            _ => false,
        }
    }
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(b),
            Value::Value(_) => self.compile().to_bool(),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            &Value::Number(_, _) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Number,
            _ => false,
        }
    }
    pub fn to_u64(self) -> Option<u64> {
        match self {
            Value::Number(i, e) => Some(i * 2u64.pow(3 as u32)),
            Value::Value(_) => self.compile().to_u64(),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            &Value::String(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::String,
            _ => false,
        }
    }
    pub fn to_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            Value::Value(_) => self.compile().to_string(),
            _ => None,
        }
    }

    pub fn is_type(&self) -> bool {
        match self {
            &Value::Type(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Type,
            _ => false,
        }
    }
    pub fn to_type(self) -> Option<Type> {
        match self {
            Value::Type(t) => Some(t),
            Value::Value(_) => self.compile().to_type(),
            _ => None,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            &Value::Ref(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Ref,
            _ => false,
        }
    }
    pub fn to_ref(self) -> Option<Ref<'a>> {
        match self {
            Value::Ref(r) => Some(r),
            Value::Value(_) => self.compile().to_ref(),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            &Value::List(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::List,
            _ => false,
        }
    }
    pub fn to_list<V>(self) -> Option<NomsList<'a, V>>
    where V: FromNoms<'a> + IntoNoms {
        match self {
            Value::List(l) => Some(NomsList::from_list(l.transform())),
            Value::Value(_) => self.compile().to_list(),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        match self {
            &Value::Map(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Map,
            _ => false,
        }
    }
    pub fn to_map<K, V>(self) -> Option<NomsMap<'a, K, V>>
    where K: FromNoms<'a> + IntoNoms + Eq + ::std::hash::Hash, V: FromNoms<'a> + IntoNoms {
        match self {
            Value::Map(m) => Some(NomsMap::from_map(m.transform())),
            Value::Value(_) => self.compile().to_map(),
            _ => None,
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
            &Value::Set(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Set,
            _ => false,
        }
    }
    pub fn to_set<V>(self) -> Option<NomsSet<'a, V>>
    where V: FromNoms<'a> + IntoNoms + Eq + ::std::hash::Hash{
        match self {
            Value::Set(s) => Some(NomsSet::from_set(s.transform())),
            Value::Value(_) => self.compile().to_set(),
            _ => None,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            &Value::Struct(_) => true,
            &Value::Value(ref chunk) => chunk.reader().read_kind() == Kind::Struct,
            _ => false,
        }
    }
    pub fn to_struct<T: NomsStruct<'a>>(self) -> Option<T> {
        match self {
            Value::Struct(Struct{ props, .. }) => T::from_prop_list(props),
            Value::Value(_) => self.compile().to_struct(),
            _ => None,
        }
    }

    pub fn compile(self) -> Self {
        match self {
            Value::Value(chunk) => chunk.reader().read_value(),
            _ => self
        }
    }

    pub fn to_chunk(self) -> Chunk<'a> {
        match self {
            Value::Value(chunk) => chunk,
            Value::Map(ref col) => Chunk::new(col.database(), self.into_noms()),
            Value::Set(ref col) => Chunk::new(col.database(), self.into_noms()),
            Value::List(ref col) => Chunk::new(col.database(), self.into_noms()),
            Value::Ref(ref col) => Chunk::new(col.database(), self.into_noms()),
            _ => Chunk::maybe(None, self.into_noms()),
        }
    }
}

impl<'a> IntoNoms for NomsValue<'a> {
    fn into_noms(&self) -> Vec<u8> {
        self.0.into_noms()
    }
}
impl<'a> FromNoms<'a> for NomsValue<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        NomsValue(Value::from_noms(chunk))
    }
}
