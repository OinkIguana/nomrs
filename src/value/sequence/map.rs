use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, MetaTuple, Sequence};

use std::collections::HashMap;
use chunk::ChunkReader;
use std::hash::Hash;
use either::Either;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsMap<K = NomsValue, V = NomsValue>(Map<K, V>)
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms;

impl<K, V> NomsMap<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    pub fn new() -> Self {
        NomsMap(Map::new())
    }

    pub(crate) fn from_map(map: Map<K, V>) -> Self {
        NomsMap(map)
    }

    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where K: ::std::borrow::Borrow<Q> {
        self.0.get(key)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Map<K = Value, V = Value>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    Inner{
        raw: Vec<MetaTuple>,
        cache: HashMap<Ref, Map<K, V>>,
    },
    Leaf{
        cache: HashMap<K, V>,
    },
}

impl<K, V> Map<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    pub fn new() -> Self {
        Map::Leaf {
            cache: HashMap::new(),
        }
    }

    pub fn transform<K2, V2>(self) -> Map<K2, V2>
    where K2: FromNoms + IntoNoms + Eq + Hash, V2: FromNoms + IntoNoms {
        match self {
            Map::Inner{ raw, cache } =>
                Map::Inner {
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            Map::Leaf{ cache } =>
                Map::Leaf {
                    cache: cache.into_iter().map(|(k, v)| (K2::from_noms(&k.into_noms()), V2::from_noms(&v.into_noms()))).collect(),
                },
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

impl<K, V> ::std::hash::Hash for Map<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl<K, V> Sequence<(K, V)> for Map<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    fn from_either(either: Either<Vec<(K, V)>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Left(it)  => Map::Leaf { cache: it.into_iter().collect() },
            Either::Right(it) => Map::Inner { raw: it, cache: HashMap::new() }
        }
    }
}

impl<K, V> IntoNoms for NomsMap<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}
impl<K, V> FromNoms for NomsMap<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    fn from_noms(v: &Vec<u8>) -> Self {
        NomsMap(ChunkReader::new(v).read_map())
    }
}
