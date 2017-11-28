use super::{varint, Value, Type, Kind};
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
    fn into_noms(&self) -> Vec<u8> {
        match self {
            &Value::Boolean(ok) => ok.into_noms(),
            // most encoders are in their own locations, except numbers are kind of special when
            // stored raw from the database
            &Value::Number(i, e) => {
                let mut bytes = Kind::Number.into_noms();
                bytes.extend(varint::encode_u64(i));
                bytes.extend(varint::encode_u64(e));
                bytes
            }
            &Value::String(ref s) => s.into_noms(),
            &Value::Value(ref chunk) => chunk.data().clone(),
            _ => vec![],
        }
    }
}
impl<'a> FromNoms<'a> for Value<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self { Value::Value(chunk.clone()) }
}

impl IntoNoms for u64 {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Number.into_noms();
        bytes.extend(varint::encode_u64(*self));
        bytes.extend(varint::encode_u64(1));
        bytes
    }
}
impl<'a> FromNoms<'a> for u64 {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_u64().unwrap()
    }
}

impl IntoNoms for bool {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Boolean.into_noms();
        bytes.push(if *self { 1 } else { 0 });
        bytes
    }
}
impl<'a> FromNoms<'a> for bool {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_bool().unwrap()
    }
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
