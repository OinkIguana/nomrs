use super::{Value, Type};
use chunk::Chunk;

pub trait IntoNoms {
    fn into_noms(&self) -> Vec<u8>;
}
pub trait FromNoms<'a> {
    fn from_noms(&Chunk<'a>) -> Self;
}

impl<'a> IntoNoms for Vec<u8> {
    fn into_noms(&self) -> Vec<u8> { self.clone() }
}
impl<'a> FromNoms<'a> for Vec<u8> {
    fn from_noms(v: &Chunk<'a>) -> Self { v.data().clone() }
}

impl<'a> IntoNoms for Value<'a> {
    fn into_noms(&self) -> Vec<u8> { unimplemented!("Trying to write {:?} to binary!", self) }
}
impl<'a> FromNoms<'a> for Value<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self { Value::Value(chunk.clone()) }
}

impl IntoNoms for String {
    fn into_noms(&self) -> Vec<u8> {
        Value::String(self.clone()).into_noms()
    }
}
impl<'a> FromNoms<'a> for String {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_string().unwrap()
    }
}

impl IntoNoms for Type {
    fn into_noms(&self) -> Vec<u8> {
        Value::Type(self.clone()).into_noms()
    }
}
impl<'a> FromNoms<'a> for Type {
    fn from_noms(chunk: &Chunk<'a>) -> Type {
        Value::from_noms(chunk).to_type().unwrap()
    }
}
