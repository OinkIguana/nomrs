use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, MetaTuple, Collection};
use database::ValueAccess;
use std::collections::HashMap;
use chunk::Chunk;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsList<'a, V = NomsValue<'a>>(List<'a, V>)
where V: FromNoms<'a> + IntoNoms;

impl<'a, V> NomsList<'a, V>
where V: FromNoms<'a> + IntoNoms {
    pub(crate) fn new(db: &'a ValueAccess) -> Self {
        NomsList(List::new(db))
    }

    pub(crate) fn from_list(list: List<'a, V>) -> Self {
        NomsList(list)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum List<'a, V = Value<'a>>
where V: FromNoms<'a> + IntoNoms {
    Inner{
        database: &'a ValueAccess,
        raw: Vec<MetaTuple<'a>>,
        cache: HashMap<Ref<'a>, List<'a, V>>,
    },
    Leaf{
        database: &'a ValueAccess,
        cache: Vec<V>,
    },
}

impl<'a, V> List<'a, V>
where V: FromNoms<'a> + IntoNoms {
    pub fn new(database: &'a ValueAccess) -> Self {
        List::Leaf{ database, cache: Vec::new() }
    }

    pub fn from_metatuples(database: &'a ValueAccess, raw: Vec<MetaTuple<'a>>) -> Self {
        List::Inner {
            database,
            raw,
            cache: HashMap::new(),
        }
    }

    pub fn from_values(database: &'a ValueAccess, raw: Vec<V>) -> Self {
        List::Leaf {
            database,
            cache: raw.into_iter().collect(),
        }
    }

    pub fn transform<V2>(self) -> List<'a, V2>
    where V2: FromNoms<'a> + IntoNoms {
        match self {
            List::Inner{ database, raw, cache } =>
                List::Inner {
                    database,
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            List::Leaf{ database, cache } =>
                List::Leaf {
                    database,
                    cache: cache.into_iter().map(|v| V2::from_noms(&Chunk::new(database, v.into_noms()))).collect(),
                },
        }
    }
}

impl<'a, V> PartialEq for List<'a, V>
where V: FromNoms<'a> + IntoNoms {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!();
    }
}
impl<'a, V> Eq for List<'a, V>
where V: FromNoms<'a> + IntoNoms {}

impl<'a, V> ::std::hash::Hash for List<'a, V>
where V: FromNoms<'a> + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl<'a, V> IntoNoms for NomsList<'a, V>
where V: FromNoms<'a> + IntoNoms {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}
impl<'a, V> FromNoms<'a> for NomsList<'a, V>
where V: FromNoms<'a> + IntoNoms {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_list().unwrap()
    }
}

impl<'a, V> Collection<'a, V> for List<'a, V>
where V: FromNoms<'a> + IntoNoms {
    fn database(&self) -> &'a ValueAccess {
        match self {
            &List::Inner{ database, .. } => database,
            &List::Leaf{ database, .. } => database,
        }
    }
}
