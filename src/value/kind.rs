//! Defines all the Noms types (kinds), and the Noms Type type
use super::{IntoNoms, FromNoms, Value};
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
    Type,
    Union,
    Hash, // internal... apparently
}
impl IntoNoms for Kind {
    fn into_noms(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}
impl<'a> FromNoms<'a> for Kind {
    fn from_noms(chunk: &Chunk<'a>) -> Self {
        chunk.reader().read_kind()
    }
}
impl Kind {
    pub fn is_primitive(self) -> bool {
        use self::Kind::*;
        match self {
            Boolean | Number | String | Blob | Value | Type => true,
            _ => false
        }
    }
}

/// Literally "type description", a `TypeDesc` describes a kind with enough detail that the type
/// could be replicated exactly in another location.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TypeDesc {
    Primitive,
    Compound(Vec<Type>),
    Struct {
        name: String,
        keys: Vec<String>,
        types: Vec<Type>,
        optional: Vec<bool>,
    }
}

/// The Noms Type type. Consists of a basic Noms kind, and a type description describing the
/// structure of more complex kinds of data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    kind: Kind,
    desc: TypeDesc,
}

impl ::std::hash::Hash for Type {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        Value::Type(self.clone()).hash(state)
    }
}

impl Type {
    pub(crate) fn primitive(kind: Kind) -> Self {
        Type {
            kind,
            desc: TypeDesc::Primitive,
        }
    }

    pub(crate) fn compound(kind: Kind, types: Vec<Type>) -> Self {
        Type {
            kind,
            desc: TypeDesc::Compound(types),
        }
    }

    pub(crate) fn structure(name: String, keys: Vec<String>, types: Vec<Type>, optional: Vec<bool>) -> Self {
        assert_eq!(keys.len(), types.len());
        assert_eq!(keys.len(), optional.len());
        Type {
            kind: Kind::Struct,
            desc: TypeDesc::Struct { name, keys, types, optional },
        }
    }
}

impl IntoNoms for Type {
    fn into_noms(&self) -> Vec<u8> {
        Value::Type(self.clone()).into_noms()
    }
}
impl<'a> FromNoms<'a> for Type {
    fn from_noms(chunk: &Chunk<'a>) -> Type {
        Value::from_noms(chunk).to_type().unwrap()
    }
}
