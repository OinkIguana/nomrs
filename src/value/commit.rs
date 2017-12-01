//! Implements the Commit type, used as the value of a dataset in the database.
use super::{Ref, NomsValue, Value, IntoNoms, FromNoms, NomsSet, NomsStruct, Empty};
use chunk::Chunk;
use std::collections::HashMap;

/// A commit from the Noms database. The value of every dataset is a commit, containing the actual
/// data from the database, along with additional arbitrary metadata and the set of parent commits.
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
    const NAME: &'static str = "Commit";

    fn from_prop_list(mut props: HashMap<String, NomsValue<'a>>) -> Option<Self> {
        Some(
            Self {
                meta: props.remove("meta")?.import().to_struct().unwrap(),
                parents: props.remove("parents")?.import().to_set().unwrap(),
                value: props.remove("value")?.transform(), // TODO: noms internal translation
            }
        )
    }

    fn to_prop_list(&self) -> HashMap<String, Vec<u8>> {
        HashMap::new()
    }
}
