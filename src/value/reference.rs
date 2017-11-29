//! The Noms Reference type
use super::{Type, Value, IntoNoms, FromNoms, Collection};
use database::ValueAccess;
use hash::{Hash, EMPTY_HASH};
use chunk::Chunk;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;

/// A Ref describes a reference within a Noms database.
///
/// TODO: type references?
#[derive(Clone)]
pub struct Ref<'a> {
    database: &'a ValueAccess,
    hash: Hash,
    value_type: Type,
    height: u64,
}

impl<'a> ::std::fmt::Debug for Ref<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Ref({})", self.hash)
    }
}

impl<'a> Ref<'a> {
    pub(crate) fn new(database: &'a ValueAccess, hash: Hash, value_type: Type, height: u64) -> Self {
        Self{ database, hash, value_type, height }
    }
    pub fn is_empty(&self) -> bool {
        self.hash == EMPTY_HASH
    }
    // TODO: this function should probably be renamed so that it does not conflict with std::hash::Hash.hash
    pub fn hash(&self) -> Hash {
        self.hash
    }
}

impl<'a> Display for Ref<'a> {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "Ref {{ hash: {} }}", self.hash)
    }
}

impl<'a> PartialEq for Ref<'a> {
    fn eq(&self, b: &Ref) -> bool {
        self.hash == b.hash
    }
}
impl<'a> Eq for Ref<'a> {}

impl<'a> ::std::hash::Hash for Ref<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
       self.hash.hash(state);
   }
}

impl<'a> IntoNoms for Ref<'a> {
    fn into_noms(&self) -> Vec<u8> {
        Value::Ref(self.clone()).into_noms()
    }
}

impl<'a> FromNoms<'a> for Ref<'a> {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        Value::from_noms(chunk).to_ref().unwrap()
    }
}

impl<'a> Collection<'a, Value<'a>> for Ref<'a> {
    fn database(&self) -> &'a ValueAccess {
        self.database
    }
}
