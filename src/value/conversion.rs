use byteorder::{NetworkEndian, ByteOrder};
use std::collections::HashMap;
use super::{Value, IntoNoms, FromNoms, Ref};
use hash::{Hash, BYTE_LEN};
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
        let mut hash = [0; BYTE_LEN];
        hash.copy_from_slice(&v.0.data()[..BYTE_LEN]);
        Ref{ hash: Hash::new(hash) }
    }
}

impl<T: FromNoms> FromNoms for HashMap<String, T> {
    fn from_noms(v: &Value) -> Self {
        println!("{:?}", v);
        unimplemented!()
    }
}
