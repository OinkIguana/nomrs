//! Defines some conversions from basic Noms types to standard Rust types
use super::{varint, Value, Kind};
use chunk::Chunk;
use util::frexp::frexp;

/// For converting from Rust types to Noms binary data
pub trait IntoNoms: ::std::fmt::Debug + Clone {
    /// Produces a unique binary representation of this value
    fn into_noms(&self) -> Vec<u8>;
}
/// For converting from Noms binary data to Rust types
pub trait FromNoms<'a>: ::std::fmt::Debug + Clone {
    /// Consumes a Chunk of data from the database and produces an actual usable value
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
            // stored raw from the database. Maybe make a type for that too (`NomsNumber`)
            &Value::Number(i, e) => {
                let mut bytes = Kind::Number.into_noms();
                bytes.extend(varint::encode_i64(i));
                bytes.extend(varint::encode_i64(e));
                bytes
            }
            &Value::String(ref s) => s.into_noms(),
            &Value::Value(ref chunk) => chunk.data().clone(),
            &Value::Ref(ref reference) => reference.into_noms(),
            &Value::Struct(ref structure) => structure.into_noms(),
            &Value::Type(ref t) => t.into_noms(),
            &Value::Union(ref v) => v.into_noms(),
            // TODO: list map set blob (needs ptree encoder)
            _ => unimplemented!("Trying to turn {:?} to bytes", self),
        }
    }
}
impl<'a> FromNoms<'a> for Value<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self { Value::Value(chunk.clone()) }
}

impl IntoNoms for u64 {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Number.into_noms();
        bytes.extend(varint::encode_i64(*self as i64));
        bytes.extend(varint::encode_i64(0i64));
        bytes
    }
}
impl<'a> FromNoms<'a> for u64 {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_u64().unwrap()
    }
}

impl IntoNoms for i64 {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Number.into_noms();
        bytes.extend(varint::encode_i64(*self));
        bytes.extend(varint::encode_i64(0i64));
        bytes
    }
}
impl<'a> FromNoms<'a> for i64 {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_i64().unwrap()
    }
}

impl IntoNoms for f64 {
    fn into_noms(&self) -> Vec<u8> {
        if *self == 0. || !self.is_finite() {
            // NaN/+-Inf not supported?!?
            return vec![Kind::Number as u8, 0, 0];
        }
        let (i, e) = frexp(*self);
        let mut bytes = Kind::Number.into_noms();
        bytes.extend(varint::encode_i64(i));
        bytes.extend(varint::encode_i64(e));
        bytes
    }
}
impl<'a> FromNoms<'a> for f64 {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_f64().unwrap()
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
        let mut bytes = Kind::String.into_noms();
        bytes.extend(varint::encode_u64(self.len() as u64));
        bytes.extend_from_slice(self.as_bytes());
        bytes
    }
}
impl<'a> FromNoms<'a> for String {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_string().unwrap()
    }
}

impl<'a> IntoNoms for &'a str {
    fn into_noms(&self) -> Vec<u8> {
        self.to_string().into_noms()
    }
}
