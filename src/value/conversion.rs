use byteorder::{NetworkEndian, ByteOrder};
use super::{Value, Type};
use chunk::Chunk;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

pub trait IntoNoms: ::std::fmt::Debug {
    fn into_noms(&self) -> Vec<u8>;
}
pub trait FromNoms: ::std::fmt::Debug {
    fn from_noms(&Vec<u8>) -> Self;
}

impl IntoNoms for Chunk {
    fn into_noms(&self) -> Vec<u8> { self.data().clone() }
}
impl FromNoms for Chunk {
    fn from_noms(v: &Vec<u8>) -> Chunk { Chunk::new(v.clone()) }
}

impl IntoNoms for Value {
    fn into_noms(&self) -> Vec<u8> { unimplemented!() }
}
impl FromNoms for Value {
    fn from_noms(v: &Vec<u8>) -> Self { Value::Value(v.clone()) }
}

impl IntoNoms for String {
    fn into_noms(&self) -> Vec<u8> {
        Value::String(self.clone()).into_noms()
    }
}
impl FromNoms for String {
    fn from_noms(v: &Vec<u8>) -> Self {
        Value::from_noms(v).to_string().unwrap()
    }
}

impl IntoNoms for Type {
    fn into_noms(&self) -> Vec<u8> {
        Value::Type(self.clone()).into_noms()
    }
}
impl FromNoms for Type {
    fn from_noms(v: &Vec<u8>) -> Type {
        Value::from_noms(v).to_type().unwrap()
    }
}
