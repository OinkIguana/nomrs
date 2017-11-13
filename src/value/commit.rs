use std::collections::{HashMap, HashSet};
use super::{Ref, Value, IntoNoms, FromNoms, Kind};

#[derive(Debug)]
pub struct Commit<M = Value, V = Value>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    meta: HashMap<String, M>,
    parents: HashSet<Ref>,
    value: V,
}

impl<M, V> IntoNoms for Commit<M, V>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    fn into_noms(&self) -> Value { unimplemented!() }
}

impl<M, V> FromNoms for Commit<M, V>
where M: IntoNoms + FromNoms, V: IntoNoms + FromNoms {
    fn from_noms(v: &Value) -> Self {
        let reader = v.0.reader();
        assert_eq!(Kind::Struct, reader.extract_kind());
        assert_eq!("Commit", reader.extract_string());
        assert_eq!(3, reader.extract_u8());
        let mut props = HashMap::new();
        for _ in 0..3 {
            let key = reader.extract_string();
            let value = reader.extract_chunk();
            props.insert(key, value);
        }
        Self {
            meta: HashMap::from_noms(&Value(props.remove("meta").unwrap())),
            parents: HashSet::from_noms(&Value(props.remove("parents").unwrap())),
            value: V::from_noms(&Value(props.remove("value").unwrap())),
        }
    }
}
