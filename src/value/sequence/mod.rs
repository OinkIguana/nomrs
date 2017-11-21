use super::{Value, Ref, FromNoms, IntoNoms};

use std::collections::{HashMap, HashSet};
use chunk::Chunk;
use std::hash::Hash;
use either::Either;

// TODO: implement standard library collection API on these types
pub(crate) trait Sequence<V> {
    fn from_either(Either<Vec<V>, Vec<MetaTuple>>) -> Self;
    fn resolve(&self, Ref) {
        unimplemented!();
    }
}

// Somethingsomething prolly tree node. See the noms source for more (meta_sequence.go).
#[derive(Clone, Debug)]
pub(crate) struct MetaTuple {
    pub reference: Ref,
    pub key: OrderedKey,
    pub num_leaves: u64,
}

// Somethingsomething key in prolly tree level. See noms source again (still meta_sequence.go)
#[derive(Clone, Debug)]
pub(crate) struct OrderedKey {
    pub is_ordered_by_value: bool,
    pub value: Option<Value>,
    pub hash: Option<::hash::Hash>,
}

#[derive(Clone, Debug)]
pub struct NomsMap<K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms>(Map<K, V>);
impl<K, V> NomsMap<K, V>
where K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms {
    pub fn new() -> Self {
        NomsMap(Map::new())
    }

    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where K: ::std::borrow::Borrow<Q> {
        self.0.get(key)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Map<K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms> {
    Inner{
        raw: Vec<MetaTuple>,
        cache: HashMap<Ref, Map<K, V>>,
    },
    Leaf{
        cache: HashMap<K, V>,
    },
}
impl<K: FromNoms + IntoNoms + Eq + Hash, V: FromNoms + IntoNoms> Map<K, V> {
    pub fn new() -> Self {
        Map::Leaf {
            cache: HashMap::new(),
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
impl<K: Eq + Hash + FromNoms + IntoNoms, V: FromNoms + IntoNoms> Sequence<(K, V)> for Map<K, V> {
    fn from_either(either: Either<Vec<(K, V)>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Left(it)  => Map::Leaf { cache: it.into_iter().collect() },
            Either::Right(it) => Map::Inner { raw: it, cache: HashMap::new() }
        }
    }
}
impl<K: IntoNoms + FromNoms + Eq + ::std::hash::Hash, V: IntoNoms + FromNoms> FromNoms for NomsMap<K, V> {
    fn from_noms(v: &Value) -> Self {
        NomsMap(v.0.reader().read_map())
    }
}
impl<K: IntoNoms + FromNoms + Eq + ::std::hash::Hash, V: IntoNoms + FromNoms> IntoNoms for NomsMap<K, V> {
    fn into_noms(&self) -> Value {
        Chunk::writer()
            .write_map(&self.0)
            .finish()
            .into_value()
    }
}

#[derive(Clone, Debug)]
pub struct NomsSet<V: Eq + Hash + FromNoms + IntoNoms>(Set<V>);

#[derive(Clone, Debug)]
pub(crate) enum Set<V: FromNoms + IntoNoms + Hash + Eq> {
    Inner(Vec<MetaTuple>),
    Leaf(HashSet<V>),
}
impl<V: FromNoms + IntoNoms + Eq + Hash> Set<V> {
    pub fn new() -> Self {
        Set::Leaf(HashSet::new())
    }
}
impl<V: Eq + Hash + FromNoms + IntoNoms> Sequence<V> for Set<V> {
    fn from_either(either: Either<Vec<V>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Left(it) => Set::Leaf(it.into_iter().collect()),
            Either::Right(it) => Set::Inner(it),
        }
    }
}
impl<V: FromNoms + IntoNoms + Hash + Eq> IntoNoms for NomsSet<V> {
    fn into_noms(&self) -> Value {
        unimplemented!()
    }
}
impl<V: FromNoms + IntoNoms + Hash + Eq> FromNoms for NomsSet<V> {
    fn from_noms(v: &Value) -> Self {
        NomsSet(v.0.reader().read_set())
    }
}

#[derive(Clone, Debug)]
pub struct NomsList<V: FromNoms + IntoNoms>(List<V>);

#[derive(Clone, Debug)]
pub(crate) enum List<V: FromNoms + IntoNoms> {
    Inner(Vec<MetaTuple>),
    Leaf(Vec<V>),
}
impl<V: FromNoms + IntoNoms> Sequence<V> for List<V> {
    fn from_either(either: Either<Vec<V>, Vec<MetaTuple>>) -> Self {
        match either {
            Either::Left(it) => List::Leaf(it.into_iter().collect()),
            Either::Right(it) => List::Inner(it),
        }
    }
}

impl<V: FromNoms + IntoNoms> IntoNoms for NomsList<V> {
    fn into_noms(&self) -> Value {
        unimplemented!()
    }
}
impl<V: FromNoms + IntoNoms> FromNoms for NomsList<V> {
    fn from_noms(v: &Value) -> Self {
        NomsList(v.0.reader().read_list())
    }
}
