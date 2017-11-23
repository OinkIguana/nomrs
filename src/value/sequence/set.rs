use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, Sequence, MetaTuple};

use chunk::ChunkReader;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use either::Either;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsSet<V = NomsValue>(Set<V>)
where V : Eq + Hash + FromNoms + IntoNoms;

impl<V> NomsSet<V>
where V: Eq + Hash + FromNoms + IntoNoms {
    pub fn new() -> Self {
        NomsSet(Set::new())
    }
    pub(crate) fn from_set(set: Set<V>) -> Self {
        NomsSet(set)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Set<V = Value>
where V: FromNoms + IntoNoms + Hash + Eq {
    Inner{
        raw: Vec<MetaTuple>,
        cache: HashMap<Ref, Set<V>>,
    },
    Leaf{
        cache: HashSet<V>
    },
}

impl<V> Set<V>
where V: FromNoms + IntoNoms + Eq + Hash{
    pub fn new() -> Self {
        Set::Leaf{ cache: HashSet::new() }
    }

    pub fn transform<V2>(self) -> Set<V2>
    where V2: FromNoms + IntoNoms + Eq + Hash {
        match self {
            Set::Inner{ raw, cache } =>
                Set::Inner{
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            Set::Leaf{ cache } =>
                Set::Leaf{
                    cache: cache.into_iter().map(|v| V2::from_noms(&v.into_noms())).collect(),
                },
        }
    }
}

impl<V> ::std::hash::Hash for Set<V>
where V: Eq + Hash + FromNoms + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl<V> Sequence<V> for Set<V>
where V: Eq + Hash + FromNoms + IntoNoms{
    fn from_either(either: Either<Vec<V>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Left(it) => Set::Leaf{ cache: it.into_iter().collect() },
            Either::Right(it) => Set::Inner{ raw: it, cache: HashMap::new() },
        }
    }
}

impl<V> IntoNoms for NomsSet<V>
where V: FromNoms + IntoNoms + Hash + Eq {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!()
    }
}
impl<V> FromNoms for NomsSet<V>
where V: FromNoms + IntoNoms + Hash + Eq {
    fn from_noms(v: &Vec<u8>) -> Self {
        NomsSet(ChunkReader::new(v).read_set())
    }
}
