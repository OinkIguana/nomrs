use super::{Value, FromNoms, IntoNoms, Sequence, MetaTuple};

use either::Either;

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
