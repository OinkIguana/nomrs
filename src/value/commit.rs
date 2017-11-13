use std::collections::HashSet;
use super::{Ref, Value, IntoNoms, FromNoms};

#[derive(Clone, Debug)]
pub struct Commit<M = Value, V = Value>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    meta: M,
    parents: HashSet<Ref>,
    value: V,
}

impl<M, V> Commit<M, V>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    pub fn value(&self) -> &V { &self.value }
    pub fn meta(&self) -> &M { &self.meta }
    pub fn parents(&self) -> &HashSet<Ref> { &self.parents }

    pub fn into_value(self) -> V { self.value }
}

impl<M, V> IntoNoms for Commit<M, V>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    fn into_noms(&self) -> Value { unimplemented!() }
}

impl<M, V> FromNoms for Commit<M, V>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    fn from_noms(v: &Value) -> Self {
        let (name, mut props) = v.0.reader().extract_struct();
        assert_eq!("Commit", name);
        Self {
            meta: M::from_noms(&Value(props.remove("meta").unwrap())),
            parents: HashSet::from_noms(&Value(props.remove("parents").unwrap())),
            value: V::from_noms(&Value(props.remove("value").unwrap())),
        }
    }
}
