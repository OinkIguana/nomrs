//! Generic representation of a value in the database

mod conversion;
mod reference;
mod commit;
mod sequence;

pub use self::reference::Ref;
pub use self::conversion::{IntoNoms, FromNoms};
pub use self::commit::Commit;
pub use self::sequence::{Map, Set, List};

pub(crate) use self::sequence::{Sequence, MetaTuple, OrderedKey};

use chunk::Chunk;

/// A C-Style enum, which must continue to be in the same order as the NomsKind enum in the
/// official Noms Go package to ensure proper deserialization.
///
/// See [noms/go/types/noms_kind.go](https://github.com/attic-labs/noms/go/types/noms_kind.go)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Hash, // internal... apparently
}
impl Kind {
    pub fn variants() -> usize {
        Kind::Union as usize + 1
    }
}

pub struct Type(Kind);

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
