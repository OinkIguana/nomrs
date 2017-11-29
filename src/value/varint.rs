//! Encoders and decoders for Google's varint encoding of integers.
//!
//! See [here](https://developers.google.com/protocol-buffers/docs/encoding) for more info on
//! the varint encoding.

/// Encodes a `u64` as a varint.
pub fn encode_u64(mut rem: u64) -> Vec<u8> {
    let mut bytes = vec![];
    while rem > 0b01111111u64 {
        bytes.push((0b11111111 & rem | 0b10000000) as u8);
        rem >>= 7;
    }
    bytes.push(rem as u8);
    bytes
}
