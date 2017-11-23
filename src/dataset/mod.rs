use database::ValueAccess;
use value::{NomsValue, Empty, Commit, Ref, IntoNoms, FromNoms, NomsStruct};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct Dataset<'a, M = Empty, V = NomsValue>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    dataset: String,
    database: &'a ValueAccess,
    reference: Ref,
    phantom_meta: PhantomData<M>,
    phantom_value: PhantomData<V>,
}

impl<'a, M, V> Dataset<'a, M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    pub(crate) fn new(database: &'a ValueAccess, dataset: &str, reference: Ref) -> Self {
        Self {
            dataset: dataset.to_string(),
            database,
            reference,
            phantom_meta: PhantomData,
            phantom_value: PhantomData,
        }
    }

    pub fn id(&self) -> &str { &self.dataset }

    pub fn head(&self) -> Option<Commit<M, V>> {
        self.database
            .get_value(self.reference.hash())
            .ok()
            .and_then(|v| v.to_struct())
    }
    pub fn head_value(&self) -> Option<V> {
        self.head().map(|c| c.into_value())
    }
    pub fn head_ref(&self) -> &Ref { &self.reference }
}

impl<'a, M, V> Debug for Dataset<'a, M, V>
where M: IntoNoms + FromNoms + NomsStruct, V: IntoNoms + FromNoms {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "Dataset(id: {:?}, ref: {:?})", self.dataset, self.reference)
    }
}
