//! Generic representation of a value in the database

use std::collections::{HashMap, HashSet};

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

pub enum Value {
    Boolean(bool),
    Number(Vec<u8>),
    String(String),
    Blob(Vec<u8>),
    Set(HashSet<Value>),
    List(Vec<Value>),
    Map(HashMap<Value, Value>),
    Union(Box<Value>),
    Ref(Ref),
    Struct(Struct),
    Optional(Option<Box<Value>>),
    Type(Type),
}

pub struct StructType(HashMap<String, Type>);

pub struct Struct(HashMap<String, Value>);

pub struct Ref {
    hash: String,
    // value: Box<Value>,
}

trait Nommable {
    fn from(Value) -> Self;
    fn to(Self) -> Value;
}
