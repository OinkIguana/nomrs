use super::{Ref, NomsValue, Value, IntoNoms, FromNoms, NomsSet, NomsStruct, Empty};
use chunk::Chunk;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Commit<'a, M = Empty, V = NomsValue<'a>>
where M: IntoNoms + FromNoms<'a> + NomsStruct<'a>, V: IntoNoms + FromNoms<'a> {
    meta: M,
    parents: NomsSet<'a, Ref<'a>>,
    value: V,
}

impl<'a, M, V> Commit<'a, M, V>
where M: IntoNoms + FromNoms<'a> + NomsStruct<'a>, V: IntoNoms + FromNoms<'a> {
    pub fn value(&self) -> &V { &self.value }
    pub fn meta(&self) -> &M { &self.meta }
    pub fn parents(&self) -> &NomsSet<Ref> { &self.parents }
    pub fn into_value(self) -> V { self.value }
}

impl<'a, M, V> IntoNoms for Commit<'a, M, V>
where M: IntoNoms + FromNoms<'a> + NomsStruct<'a>, V: IntoNoms + FromNoms<'a> {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}

impl<'a, M, V> FromNoms<'a> for Commit<'a, M, V>
where M: IntoNoms + FromNoms<'a> + NomsStruct<'a>, V: IntoNoms + FromNoms<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_struct().unwrap()
    }
}

impl<'a, M, V> NomsStruct<'a> for Commit<'a, M, V>
where M: IntoNoms + FromNoms<'a> + NomsStruct<'a>, V: IntoNoms + FromNoms<'a> {
    fn from_prop_list(mut props: HashMap<String, NomsValue<'a>>) -> Option<Self> {
        let meta = props.remove("meta");
        let parents = props.remove("parents");
        let value = props.remove("value");
        match (meta, parents, value) {
            (Some(meta), Some(parents), Some(value)) =>
                Some(
                    Self {
                        meta: meta.import().to_struct().unwrap(),
                        parents: parents.import().to_set().unwrap(),
                        value: value.transform(), // TODO: noms internal translation
                    }
                ),
            _ => None,
        }
    }
}
