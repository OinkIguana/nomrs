//! Performs the serialization/deserialization of data that is to be sent to and from the Noms
//! instance

use chunk::Chunk;

/// Serializes data according to the format defined by the NomsDB
pub fn serialize(value: Chunk) -> Vec<u8> {
    // TODO: implement
    unimplemented!()
}

/// Deserializes data according to the format defined by the NomsDB
pub fn deserialize(bytes: Vec<u8>) -> Chunk {
    // TODO: implement
    unimplemented!()
}
