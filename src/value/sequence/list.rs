use super::{NomsValue, Value, Ref, FromNoms, IntoNoms, Sequence, MetaTuple};

use std::collections::HashMap;
use either::Either;
use chunk::ChunkReader;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NomsList<V = NomsValue>(List<V>)
where V: FromNoms + IntoNoms;

impl<V> NomsList<V>
where V: FromNoms + IntoNoms {
    pub fn new() -> Self {
        NomsList(List::new())
    }

    pub(crate) fn from_list(list: List<V>) -> Self {
        NomsList(list)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum List<V = Value>
where V: FromNoms + IntoNoms {
    Inner{
        raw: Vec<MetaTuple>,
        cache: HashMap<Ref, List<V>>,
    },
    Leaf{
        cache: Vec<V>,
    },
}

impl<V> List<V>
where V: FromNoms + IntoNoms {
    pub fn new() -> Self {
        List::Leaf{ cache: Vec::new() }
    }

    pub fn transform<V2>(self) -> List<V2>
    where V2: FromNoms + IntoNoms {
        match self {
            List::Inner{ raw, cache } =>
                List::Inner {
                    raw,
                    cache: cache.into_iter().map(|(k, v)| (k, v.transform())).collect(),
                },
            List::Leaf{ cache } =>
                List::Leaf {
                    cache: cache.into_iter().map(|v| V2::from_noms(&v.into_noms())).collect(),
                },
        }
    }
}

impl<V> Sequence<V> for List<V>
where V: FromNoms + IntoNoms {
    fn from_either(either: Either<Vec<V>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Right(it) => List::Inner {
                raw: it,
                cache: HashMap::new(),
            },
            Either::Left(it) => List::Leaf {
                cache: it.into_iter().collect(),
            },
        }
    }
}

impl<V> ::std::hash::Hash for List<V>
where V: FromNoms + IntoNoms {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl<V> IntoNoms for NomsList<V>
where V: FromNoms + IntoNoms {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!()
    }
}
impl<V> FromNoms for NomsList<V>
where V: FromNoms + IntoNoms {
    fn from_noms(c: &Vec<u8>) -> Self {
        NomsList(ChunkReader::new(c).read_list())
    }
}
