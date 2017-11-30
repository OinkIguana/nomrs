//! Defines all the Noms types (kinds), and the Noms Type type
use super::{IntoNoms, FromNoms, Value, varint};
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
impl TypeDesc {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            &TypeDesc::Primitive => vec![],
            &TypeDesc::Compound(ref types) => {
                let mut bytes = if types.len() > 1 {
                    varint::encode_u64(types.len() as u64)
                } else { vec![] };
                for t in types {
                    bytes.extend(t.desc.to_bytes());
                }
                bytes
            }
            &TypeDesc::Struct{ ref name, ref keys, ref types, ref optional } => {
                let mut bytes = varint::encode_u64(name.len() as u64);
                bytes.extend(name.as_bytes());
                bytes.extend(varint::encode_u64(keys.len() as u64));
                for key in keys {
                    bytes.extend(varint::encode_u64(key.len() as u64));
                    bytes.extend(key.as_bytes());
                }
                for t in types {
                    bytes.extend(t.to_bytes());
                }
                for o in optional {
                    bytes.push(if *o { 1 } else { 0 });
                }
                bytes
            }
        }
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

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.kind as u8];
        bytes.extend(self.desc.to_bytes());
        bytes
    }
}

impl IntoNoms for Type {
    fn into_noms(&self) -> Vec<u8> {
        let mut bytes = Kind::Type.into_noms();
        bytes.extend(self.to_bytes());
        bytes
    }
}

impl<'a> FromNoms<'a> for Type {
    fn from_noms(chunk: &Chunk<'a>) -> Type {
        Value::from_noms(chunk).to_type().unwrap()
    }
}
