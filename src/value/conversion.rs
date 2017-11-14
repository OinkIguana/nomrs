use byteorder::{NetworkEndian, ByteOrder};
use std::collections::{HashMap, HashSet};
use super::{Value, Type};
use chunk::Chunk;

pub trait IntoNoms {
    fn into_noms(&self) -> Value;
}
pub trait FromNoms {
    fn from_noms(&Value) -> Self;
}

impl IntoNoms for Chunk {
    fn into_noms(&self) -> Value { self.clone().into_value() }
}
impl FromNoms for Chunk {
    fn from_noms(v: &Value) -> Chunk { v.0.clone() }
}

impl IntoNoms for Value {
    fn into_noms(&self) -> Value { self.clone() }
}
impl FromNoms for Value {
    fn from_noms(v: &Value) -> Self { v.clone() }
}

impl<T: IntoNoms> IntoNoms for Vec<T> {
    fn into_noms(&self) -> Value {
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, self.len() as u32);
        let mut val = buf.to_vec();
        val.extend(self.iter().flat_map(|v| v.into_noms().into_raw().into_iter()));
        Value(Chunk::new(val))
    }
}

impl<K: FromNoms + Eq + ::std::hash::Hash, V: FromNoms> FromNoms for HashMap<K, V> {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_map()
    }
}

impl<K: IntoNoms + Eq + ::std::hash::Hash, V: IntoNoms> IntoNoms for HashMap<K, V> {
    fn into_noms(&self) -> Value {
        Chunk::writer()
            .write_map(self)
            .finish()
            .into_value()
    }
}

impl<V: IntoNoms + Eq + ::std::hash::Hash> IntoNoms for HashSet<V> {
    fn into_noms(&self) -> Value {
        unimplemented!()
    }
}
impl<V: FromNoms + Eq + ::std::hash::Hash> FromNoms for HashSet<V> {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_set()
    }
}

impl FromNoms for String {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_string()
    }
}
impl IntoNoms for String {
    fn into_noms(&self) -> Value {
        Chunk::writer()
            .write_string(self)
            .finish()
            .into_value()
    }
}

impl IntoNoms for Type {
    fn into_noms(&self) -> Value {
        Chunk::writer()
            .write_kind(self.0)
            .finish()
            .into_value()
    }
}
impl FromNoms for Type {
    fn from_noms(v: &Value) -> Type { Type(v.0.reader().extract_kind()) }
}
