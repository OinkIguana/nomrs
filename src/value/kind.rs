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
impl Kind {
    pub fn variants() -> usize {
        Kind::Union as usize + 1
    }

    pub fn is_primitive(self) -> bool {
        use self::Kind::*;
        match self {
            Boolean | Number | String | Blob | Value | Type => true,
            _ => false
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    kind: Kind,
    desc: TypeDesc,
}
impl ::std::hash::Hash for Type {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        unimplemented!();
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
