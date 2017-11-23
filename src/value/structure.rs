use std::collections::HashMap;
use super::{NomsValue, Value, FromNoms, IntoNoms};

pub trait NomsStruct: Sized {
    fn from_prop_list(props: HashMap<String, NomsValue>) -> Option<Self>;
}

#[derive(Clone, Debug)]
pub(crate) struct Struct {
    pub name: String,
    pub props: HashMap<String, NomsValue>,
}

impl Eq for Struct {}
impl PartialEq for Struct {
    fn eq(&self, other: &Struct) -> bool {
        false
    }
}

impl ::std::hash::Hash for Struct {
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
impl FromNoms for Empty {
    fn from_noms(v: &Vec<u8>) -> Self {
        Value::from_noms(v).to_struct().unwrap()
    }
}
impl NomsStruct for Empty {
    fn from_prop_list(_: HashMap<String, NomsValue>) -> Option<Self> { Some(Empty) }
}
