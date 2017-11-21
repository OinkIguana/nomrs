use super::{Value, Ref, FromNoms, IntoNoms, MetaTuple, Sequence};

use std::collections::HashMap;
use chunk::Chunk;
use std::hash::Hash;
use either::Either;


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
