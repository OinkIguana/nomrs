use std::collections::HashMap;
use chunk::Chunk;
use super::{NomsValue, Value, FromNoms, IntoNoms};

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
        false
    }
}

impl<'a> ::std::hash::Hash for Struct<'a> {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Empty;
impl IntoNoms for Empty {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
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
