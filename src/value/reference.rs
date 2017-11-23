use super::{Type, Value, IntoNoms, FromNoms};
use hash::{Hash, EMPTY_HASH};
use chunk::Chunk;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std;

#[derive(Clone, Debug)]
pub struct Ref {
    hash: Hash,
    value_type: Type,
    height: u64,
}

impl Ref {
    pub fn new(hash: Hash, value_type: Type, height: u64) -> Self {
        Self{ hash, value_type, height }
    }
    pub fn is_empty(&self) -> bool {
        self.hash == EMPTY_HASH
    }
    pub fn hash(&self) -> Hash {
        self.hash
    }
}

impl Display for Ref {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ref {{ hash: {} }}", self.hash)
    }
}

impl PartialEq for Ref {
    fn eq(&self, b: &Ref) -> bool {
        self.hash == b.hash
    }
}
impl Eq for Ref {}

impl std::hash::Hash for Ref {
    fn hash<H: Hasher>(&self, state: &mut H) {
       self.hash.hash(state);
   }
}

impl IntoNoms for Ref {
    fn into_noms(&self) -> Vec<u8> {
        Value::Ref(self.clone()).into_noms()
    }
}

impl FromNoms for Ref {
    fn from_noms(v: &Vec<u8>) -> Self {
        Value::from_noms(v).to_ref().unwrap()
    }
}
