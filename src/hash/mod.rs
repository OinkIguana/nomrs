//! Helpers for dealing with hashes

use error::Error;
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use data_encoding::{Encoding, Specification};

lazy_static! {
    static ref HASH_FORMAT: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("0123456789abcdefghijklmnopqrstuv");
        spec.encoding().unwrap()
    };
}
const OUTPUT_BYTES: usize = 64;
/// Number of bytes used to represent the hash
pub const BYTE_LEN: usize = 20;
/// Number of characters used to represent the hash in a base32 string
pub const STRING_LEN: usize = 32;

pub const EMPTY_HASH: Hash = Hash([0; BYTE_LEN]);
/// Representation of a hash
#[derive(PartialEq, Eq, Clone, Copy, Debug, Ord, PartialOrd)]
pub struct Hash([u8; BYTE_LEN]);
impl Hash {
    pub fn new(v: [u8; BYTE_LEN]) -> Self { Hash(v) }
    pub fn from_slice(v: &[u8]) -> Self {
        let Hash(mut hash) = EMPTY_HASH;
        hash.copy_from_slice(&v[..BYTE_LEN]);
        Hash(hash)
    }
    pub fn to_string(&self) -> String {
        HASH_FORMAT.encode(&self.0)
    }
    pub fn raw_bytes(&self) -> [u8; BYTE_LEN] {
        self.0
    }

    pub fn from_string(base32: String) -> Result<Hash, Error> {
        let mut hash = [0; BYTE_LEN];
        let len = HASH_FORMAT.decode_mut(base32.as_bytes(), &mut hash)?;
        assert_eq!(BYTE_LEN, len);
        Ok(Hash(hash))
    }

    pub fn is_empty(&self) -> bool {
        *self == EMPTY_HASH
    }
}

impl ::std::hash::Hash for Hash {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl ::std::fmt::Display for Hash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(f, "{}", self.to_string())
    }
}

/// Produces the hash of an array of bytes
// TODO: ensure this produces the same sort of hashes as the Go hasher
pub fn hash(input: &[u8]) -> Hash {
    let mut hasher = Sha512::new();
    hasher.input(input);
    let mut buf = [0; OUTPUT_BYTES];
    hasher.result(&mut buf);
    let mut hash = [0; BYTE_LEN];
    hash.copy_from_slice(&buf[..BYTE_LEN]);
    Hash(hash)
}
