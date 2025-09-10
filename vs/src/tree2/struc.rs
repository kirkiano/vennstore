use std::{fmt,
          path::{Path, PathBuf}};

use crate::Staging;


#[derive(Debug, Clone)]
pub struct Tree2(PathBuf);


impl fmt::Display for Tree2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hash tree at {}", self.0.display())
    }
}


impl From<PathBuf> for Tree2 {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}


impl AsRef<Path> for Tree2 {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}


impl Tree2 {
    pub fn staging(&self) -> Staging {
        let mut p = self.0.clone();
        p.push(".staging");
        p.into()
    }
}