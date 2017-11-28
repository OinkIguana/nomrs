pub fn encode_u64(mut rem: u64) -> Vec<u8> {
    let mut bytes = vec![];
    while rem > 0b01111111u64 {
        bytes.push((0b11111111 & rem | 0b10000000) as u8);
        rem >>= 7;
    }
    bytes.push(rem as u8);
    bytes
}
