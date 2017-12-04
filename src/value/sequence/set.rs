use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, MetaTuple, Collection};
use database::ChunkStore;
use chunk::Chunk;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsSet<'a, V = NomsValue<'a>>(Set<'a, V>)
where V : Eq + Hash + FromNoms<'a> + IntoNoms;

impl<'a, V> NomsSet<'a, V>
where V: Eq + Hash + FromNoms<'a> + IntoNoms {
    pub(crate) fn new(db: &'a ChunkStore) -> Self {
        NomsSet(Set::new(db))
    }

    pub(crate) fn from_set(set: Set<'a, V>) -> Self {
        NomsSet(set)
    }

    pub fn to_set(&self) -> HashSet<V> {
        self.0.to_set()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Set<'a, V = Value<'a>>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {
    Inner{
        database: &'a ChunkStore,
        raw: Vec<MetaTuple<'a>>,
        cache: HashMap<Ref<'a>, Set<'a, V>>,
    },
    Leaf{
        database: &'a ChunkStore,
        cache: HashSet<V>
    },
}

impl<'a, V> PartialEq for Set<'a, V>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!();
    }
}
impl<'a, V> Eq for Set<'a, V>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {}

impl<'a, V> Set<'a, V>
where V: FromNoms<'a> + IntoNoms + Eq + Hash{
    pub fn new(database: &'a ChunkStore) -> Self {
        Set::Leaf{ database, cache: HashSet::new() }
    }

    pub fn from_metatuples(database: &'a ChunkStore, raw: Vec<MetaTuple<'a>>) -> Self {
        Set::Inner {
            database,
            raw,
            cache: HashMap::new(),
        }
    }

    pub fn from_values(database: &'a ChunkStore, raw: Vec<V>) -> Self {
        Set::Leaf {
            database,
            cache: raw.into_iter().collect(),
        }
    }

    pub fn to_set(&self) -> HashSet<V> {
        match self {
            &Set::Leaf{ ref cache, .. } => cache.clone(),
            &Set::Inner{ ref raw, .. } =>
                self
                    .resolve_all(raw)
                    .unwrap()
                    .into_iter()
                    .flat_map(|v| v.to_set())
                    .collect()
        }
    }

    pub fn transform<V2>(self) -> Set<'a, V2>
    where V2: FromNoms<'a> + IntoNoms + Eq + Hash {
        match self {
            Set::Inner{ database, raw, cache } =>
                Set::Inner{
                    database,
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            Set::Leaf{ database, cache } =>
                Set::Leaf{
                    database,
                    cache: cache.into_iter().map(|v| V2::from_noms(&Chunk::new(database, v.into_noms()))).collect(),
                },
        }
    }
}

impl<'a, V> ::std::hash::Hash for Set<'a, V>
where V: Eq + Hash + FromNoms<'a> + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl<'a, V> IntoNoms for NomsSet<'a, V>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!()
    }
}
impl<'a, V> FromNoms<'a> for NomsSet<'a, V>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_set().unwrap()
    }
}

impl<'a, V> Collection<'a, NomsSet<'a, V>> for Set<'a, V>
where V: FromNoms<'a> + IntoNoms + Hash + Eq {
    fn database(&self) -> &'a ChunkStore {
        match self {
            &Set::Inner{ database, .. } => database,
            &Set::Leaf{ database, .. } => database,
        }
    }
}
