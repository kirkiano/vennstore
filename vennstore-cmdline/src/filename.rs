use core::fmt;
use std::{path::{Path, PathBuf},
          hash::{Hash, Hasher, DefaultHasher}};

use crate::constants::FILE_DIR;


/// A file to be stored
///
/// This should be the filename only, not a path.
/// It should also be unique.
#[derive(Debug, Clone)]
pub struct Filename(String);


impl From<String> for Filename {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Filename> for String {
    fn from(value: Filename) -> Self {
        value.0
    }
}


impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}


impl Filename {

    /// The path at which this file should be placed
    pub fn hashed_path(&self) -> PathBuf {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        let hash_value: u64 = h.finish();
        let (m, n) = Filename::hash_to_bucket(hash_value);

        Path::new(FILE_DIR)
            .join(m.to_string())
            .join(n.to_string())
            .join(self.0.clone())
            .to_path_buf()
    }

    fn hash_to_bucket(h: u64) -> (u8, u8) {
        let b = h % 10_000;
        let bn: u64 = b / 100;
        let bnk = bn * 100;
        let bm = b - bnk;
        (bn as u8, bm as u8)
    }
}
