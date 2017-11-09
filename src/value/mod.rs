//! Generic representation of a value in the database

use std::collections::{HashSet, HashMap};

pub enum Type {
    Boolean,
    Number,
    String,
    Blob,
    Set(Box<Type>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Union(Vec<Type>),
    Ref,
    Struct(StructType),
    Type,
    Nullable(Box<Type>),
}

#[allow(dead_code)]
pub struct Ref {
    hash: String,
    type_: Type,
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
    Type(Type),
    Nullable(Option<Box<Value>>),
}

pub struct StructType(HashMap<String, Type>);
pub struct Struct(HashMap<String, Value>);
