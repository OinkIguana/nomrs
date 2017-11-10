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

/// Representation of a hash
pub type Hash = [u8; BYTE_LEN];
pub const EMPTY_HASH: Hash = [0; BYTE_LEN];

/// Produces the hash of an array of bytes
// TODO: ensure this produces the same sort of hashes as the Go hasher
pub fn hash<'a>(input: &'a [u8]) -> Hash {
    let mut hasher = Sha512::new();
    hasher.input(input);
    let mut buf = [0; OUTPUT_BYTES];
    hasher.result(&mut buf);
    let mut hash: Hash = [0; BYTE_LEN];
    hash.copy_from_slice(&buf[..BYTE_LEN]);
    hash
}

pub fn parse(base32: &[u8]) -> Result<Hash, Error> {
    let mut hash: Hash = [0; BYTE_LEN];
    let len = HASH_FORMAT.decode_mut(base32, &mut hash)?;
    assert_eq!(BYTE_LEN, len);
    Ok(hash)
}
