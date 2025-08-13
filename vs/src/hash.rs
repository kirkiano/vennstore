use std::{fmt, path::PathBuf};

use generic_array::{GenericArray, typenum::U20};
use sha1::{Sha1, Digest};
use rusqlite::{Result, ToSql,
               types::{Value, ToSqlOutput}};

use util::{Filename, ProperPathBuf};



type Inner = [u8; 3]; // TODO: make 3 a constant parameter


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash(Inner);


impl From<&Filename> for Hash {
    fn from(f: &Filename) -> Self {
        let bytes: &[u8] = f.as_ref();
        bytes.into()
    }
}


impl From<&[u8]> for Hash {
    fn from(bytes: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(bytes);
        let result: GenericArray<u8, U20> = hasher.finalize();
        let inner: Inner = [result[0], result[1], result[2]];
        Self(inner)
    }
}


impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}


impl Hash {
    pub fn path(&self) -> ProperPathBuf {
        let parts = self.0.iter().map(|b| format!("{:02x}", b));
        PathBuf::from_iter(parts).try_into().unwrap()
    }

    pub fn filename_path(f: &Filename) -> ProperPathBuf {
        Self::from(f).path() + f
    }
}


/////////////////////////////////////////////////////////////////////
/// sqlite

impl ToSql for Hash {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let v = Value::Text(self.to_string());
        Ok(ToSqlOutput::Owned(v))
    }
}
