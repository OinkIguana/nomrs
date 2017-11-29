//! The Noms Struct type
use std::collections::HashMap;
use chunk::Chunk;
use super::{varint, NomsValue, Value, FromNoms, IntoNoms, Kind};

pub trait NomsStruct<'a>: Sized {
    fn from_prop_list(props: HashMap<String, NomsValue<'a>>) -> Option<Self>;
}

#[derive(Clone, Debug)]
pub(crate) struct Struct<'a> {
    pub name: String,
    pub props: HashMap<String, NomsValue<'a>>,
}

impl<'a> Eq for Struct<'a> {}
impl<'a> PartialEq for Struct<'a> {
    fn eq(&self, other: &Struct) -> bool {
        self.name != other.name || self.props.len() != other.props.len() && {
            for (key, value) in &self.props {
                if other.props.get(key) != Some(value) {
                    return false
                }
            }
            true
        }
    }
}

impl<'a> IntoNoms for Struct<'a> {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Struct.into_noms();
        bytes.extend(varint::encode_u64(self.name.len() as u64));
        bytes.extend(self.name.as_bytes());
        bytes.extend(varint::encode_u64(self.props.len() as u64));
        for (key, value) in &self.props {
            bytes.extend(varint::encode_u64(key.len() as u64));
            bytes.extend(key.as_bytes());
            bytes.extend(value.into_noms());
        }
        bytes
    }
}
impl<'a> FromNoms<'a> for Struct<'a> {
    fn from_noms(_chunk: &Chunk) -> Self {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Empty;
impl IntoNoms for Empty {
    fn into_noms(&self) -> Vec<u8> {
        Struct{ name: "".to_string(), props: HashMap::new() }.into_noms()
    }
}
impl<'a> FromNoms<'a> for Empty {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_struct().unwrap()
    }
}
impl<'a> NomsStruct<'a> for Empty {
    fn from_prop_list(_: HashMap<String, NomsValue>) -> Option<Self> { Some(Empty) }
}
