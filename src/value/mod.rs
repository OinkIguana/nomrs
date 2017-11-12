//! Generic representation of a value in the database

mod conversion;
use std::collections::{HashMap, HashSet};
use hash::{Hash, EMPTY_HASH};

pub enum Type {
    Boolean,
    Number,
    String,
    Blob,
    Set(Box<Type>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Union(HashSet<Type>),
    Ref(Box<Type>),
    Struct(StructType),
    Optional(Box<Type>),
    Type,
}

// TODO: this representation of value is probably wrong, and all of this will just be raw bytes
//       with conversions defined instead
// pub enum Value {
//     Boolean(bool),
//     Number(Vec<u8>),
//     String(String),
//     Blob(Vec<u8>),
//     Set(HashSet<Value>),
//     List(Vec<Value>),
//     Map(HashMap<Value, Value>),
//     Union(Box<Value>),
//     Ref(Ref),
//     Struct(Struct),
//     Optional(Option<Box<Value>>),
//     Type(Type),
// }

pub type Value = Vec<u8>;

pub struct StructType(HashMap<String, Type>);

pub struct Struct(HashMap<String, Value>);

pub struct Commit {
    meta: Value,
    parents: Value,
    value: Value,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref {
    hash: Hash,
    // value: Box<Value>,
}
impl Ref {
    pub fn new(hash: Hash) -> Self {
        Self{ hash }
    }
    pub fn is_empty(&self) -> bool {
        self.hash == EMPTY_HASH
    }
    pub fn hash(&self) -> Hash {
        self.hash
    }
}

pub trait IntoNoms {
    fn into_noms(&self) -> Value;
}
pub trait FromNoms {
    fn from_noms(&Value) -> Self;
}
