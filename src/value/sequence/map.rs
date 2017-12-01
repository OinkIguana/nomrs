use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, MetaTuple, Collection};
use database::ValueAccess;
use std::collections::HashMap;
use chunk::Chunk;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsMap<'a, K = NomsValue<'a>, V = NomsValue<'a>>(Map<'a, K, V>)
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms;

impl<'a, K, V> NomsMap<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    pub(crate) fn new(db: &'a ValueAccess) -> Self {
        NomsMap(Map::new(db))
    }

    pub(crate) fn from_map(map: Map<'a, K, V>) -> Self {
        NomsMap(map)
    }

    pub fn to_map(&self) -> HashMap<K, V> {
        self.0.to_map()
    }

    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where K: ::std::borrow::Borrow<Q> {
        self.0.get(key)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Map<'a, K = Value<'a>, V = Value<'a>>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    Inner{
        database: &'a ValueAccess,
        raw: Vec<MetaTuple<'a>>,
        cache: HashMap<Ref<'a>, Map<'a, K, V>>,
    },
    Leaf{
        database: &'a ValueAccess,
        cache: HashMap<K, V>,
    },
}

impl<'a, K, V> Map<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    pub fn new(database: &'a ValueAccess) -> Self {
        Map::Leaf {
            database,
            cache: HashMap::new(),
        }
    }

    pub fn from_metatuples(database: &'a ValueAccess, raw: Vec<MetaTuple<'a>>) -> Self {
        Map::Inner {
            database,
            raw,
            cache: HashMap::new(),
        }
    }

    pub fn from_values(database: &'a ValueAccess, raw: Vec<(K, V)>) -> Self {
        Map::Leaf {
            database,
            cache: raw.into_iter().collect(),
        }
    }

    pub fn transform<K2, V2>(self) -> Map<'a, K2, V2>
    where K2: FromNoms<'a> + IntoNoms + Eq + Hash, V2: FromNoms<'a> + IntoNoms {
        match self {
            Map::Inner{ database, raw, cache } =>
                Map::Inner {
                    database,
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            Map::Leaf{ database, cache } =>
                Map::Leaf {
                    database,
                    cache: cache.into_iter().map(|(k, v)|
                        ( K2::from_noms(&Chunk::new(database, k.into_noms()))
                        , V2::from_noms(&Chunk::new(database, v.into_noms())))
                    ).collect(),
                },
        }
    }

    pub fn to_map(&self) -> HashMap<K, V> {
        match self {
            &Map::Leaf{ ref cache, .. } => cache.clone(),
            &Map::Inner{ ref raw, .. } =>
                self
                    .resolve_all(raw)
                    .unwrap()
                    .into_iter()
                    .flat_map(|v| v.to_map())
                    .collect()
        }
    }

    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where K: ::std::borrow::Borrow<Q> {
        match self {
            &Map::Inner { .. }           => unimplemented!(),
            &Map::Leaf { ref cache, .. } => cache.get(key)
        }
    }
}

impl<'a, K, V> PartialEq for Map<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!();
    }
}
impl<'a, K, V> Eq for Map<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {}

impl<'a, K, V> ::std::hash::Hash for Map<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {
        unimplemented!();
    }
}

impl<'a, K, V> IntoNoms for NomsMap<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}
impl<'a, K, V> FromNoms<'a> for NomsMap<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_map().unwrap()
    }
}

impl<'a, K, V> Collection<'a, NomsMap<'a, K, V>> for Map<'a, K, V>
where K: FromNoms<'a> + IntoNoms + Eq + Hash, V: FromNoms<'a> + IntoNoms {
    fn database(&self) -> &'a ValueAccess {
        match self {
            &Map::Inner{ database, .. } => database,
            &Map::Leaf{ database, .. } => database,
        }
    }
}
