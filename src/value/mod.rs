//! Generic representation of a value in the database

mod conversion;
mod reference;
mod commit;

pub use self::reference::Ref;
pub use self::conversion::{IntoNoms, FromNoms};
pub use self::commit::Commit;

use chunk::Chunk;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    Boolean,
    Number,
    String,
    Blob,
    Value,
    List,
    Map,
    Ref,
    Set,
    Struct,
    Cycle,
    Union,
}
impl Kind {
    pub fn variants() -> usize {
        Kind::Union as usize + 1
    }
}

pub struct Type(Kind); // TODO: Figure out if this is needed?

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Value(pub(crate) Chunk);
impl Value {
    pub fn raw(&self) -> &Vec<u8> {
        self.0.data()
    }
    pub fn into_raw(self) -> Vec<u8> {
        self.0.into_data()
    }
}
