use super::{Value, IntoNoms, FromNoms};
use hash::{Hash, EMPTY_HASH};
use chunk::Chunk;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std;

#[derive(Clone, Debug)]
pub struct Ref {
    hash: Hash,
}

impl Ref {
    pub fn new(hash: Hash) -> Self {
        Self{ hash }
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
        Chunk::writer()
            .write_ref(&self)
            .finish()
            .into_value()
    }
}

impl FromNoms for Ref {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_ref()
    }
}
