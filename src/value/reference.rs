use super::{Value, IntoNoms, FromNoms};
use hash::{Hash, EMPTY_HASH};
use chunk::Chunk;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std;

#[derive(Clone, Debug)]
pub struct Ref {
    hash: Hash,
    value: Option<Box<Value>>,
}

impl Ref {
    pub fn new(hash: Hash) -> Self {
        Self{ hash, value: None }
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
    fn into_noms(&self) -> Value {
        Value(Chunk::new(self.hash.raw_bytes().to_vec()))
    }
}

impl FromNoms for Ref {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_ref()
    }
}
