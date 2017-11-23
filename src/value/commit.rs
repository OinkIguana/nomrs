use super::{Ref, NomsValue, Value, IntoNoms, FromNoms, NomsSet, NomsStruct, Empty};
use chunk::Chunk;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Commit<M = Empty, V = NomsValue>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    meta: M,
    parents: NomsSet<Ref>,
    value: V,
}

impl<M, V> Commit<M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    pub fn value(&self) -> &V { &self.value }
    pub fn meta(&self) -> &M { &self.meta }
    pub fn parents(&self) -> &NomsSet<Ref> { &self.parents }
    pub fn into_value(self) -> V { self.value }
}

impl<M, V> IntoNoms for Commit<M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}

impl<M, V> FromNoms for Commit<M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    fn from_noms(v: &Vec<u8>) -> Self {
        Value::from_noms(v).to_struct().unwrap()
    }
}

impl<M, V> NomsStruct for Commit<M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    fn from_prop_list(mut props: HashMap<String, NomsValue>) -> Option<Self> {
        let meta = props.remove("meta");
        let parents = props.remove("parents");
        let value = props.remove("value");
        match (meta, parents, value) {
            (Some(meta), Some(parents), Some(value)) =>
                Some(
                    Self {
                        meta: meta.import().to_struct().unwrap(),
                        parents: parents.import().to_set().unwrap(),
                        value: V::from_noms(&value.into_noms()), // TODO: noms internal translation
                    }
                ),
            _ => None,
        }
    }
}
