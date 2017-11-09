//! Helpers for dealing with hashes

use crypto::digest::Digest;
use crypto::sha2::Sha512;

const OUTPUT_BYTES: usize = 64;

/// Number of bytes used to represent the hash
pub const BYTE_LEN: usize = 20;

/// Representation of a hash
pub type Hash = [u8; BYTE_LEN];

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
