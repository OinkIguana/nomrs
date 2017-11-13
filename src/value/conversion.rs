use byteorder::{NetworkEndian, ByteOrder};
use std::collections::HashMap;
use super::{Value, IntoNoms, FromNoms, Ref, Kind};
use chunk::Chunk;

impl<T: IntoNoms> IntoNoms for Vec<T> {
    fn into_noms(&self) -> Value {
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, self.len() as u32);
        let mut val = buf.to_vec();
        val.extend(self.iter().flat_map(|v| v.into_noms().into_raw().into_iter()));
        Value(Chunk::new(val))
    }
}

impl IntoNoms for Ref {
    fn into_noms(&self) -> Value {
        Value(Chunk::new(self.hash.raw_bytes().to_vec()))
    }
}

impl FromNoms for Ref {
    fn from_noms(v: &Value) -> Self {
        v.0.reader().extract_ref()
    }
}

impl<T: FromNoms> FromNoms for HashMap<String, T> {
    fn from_noms(v: &Value) -> Self {
        let mut map = HashMap::new();
        let reader = v.0.reader();
        assert_eq!(Kind::Map, reader.extract_kind());
        reader.skip(1); // idk what this byte is for yet
        let entries = reader.extract_u8();
        for _ in 0..entries {
            let key = reader.extract_string();
            let value = reader.extract_chunk();
            map.insert(key, T::from_noms(&Value(value)));
        }
        map
    }
}
