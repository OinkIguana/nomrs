use super::{Value, FromNoms, IntoNoms, Sequence, MetaTuple};

use std::collections::HashSet;
use std::hash::Hash;
use either::Either;

#[derive(Clone, Debug)]
pub struct NomsSet<V>(Set<V>) where V : Eq + Hash + FromNoms + IntoNoms;
impl<V> NomsSet<V>
where V: Eq + Hash + FromNoms + IntoNoms {
    pub fn new() -> Self {
        NomsSet(Set::new())
    }
}

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
