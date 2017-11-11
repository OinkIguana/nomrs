use super::{IntoNoms, FromNoms, Value, Ref};
use hash::{Hash, BYTE_LEN};
use byteorder::{NetworkEndian, ByteOrder};
use std::collections::HashMap;

impl<T: IntoNoms> IntoNoms for Vec<T> {
    fn into_noms(&self) -> Value {
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, self.len() as u32);
        let mut val = buf.to_vec();
        val.extend(self.iter().flat_map(|v| v.into_noms().into_iter()));
        val
    }
}

impl IntoNoms for Ref {
    fn into_noms(&self) -> Value {
        self.hash.to_vec()
    }
}

impl FromNoms for Ref {
    fn from_noms(v: &Value) -> Self {
        let mut hash: Hash = [0; BYTE_LEN];
        hash.copy_from_slice(&v[..BYTE_LEN]);
        Ref{ hash }
    }
}

impl<T: FromNoms> FromNoms for HashMap<String, T> {
    fn from_noms(v: &Value) -> Self {
        unimplemented!()
    }
}
