use std::{fmt, path::{Path, PathBuf}};

use util::Filename;
use crate::{FILES_DIR_NAME, SQLITE_DB_NAME};



/// The root of the file store
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootPath2(PathBuf);


impl fmt::Display for RootPath2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p: &Path = self.0.as_ref();
        p.display().fmt(f)
    }
}


impl From<PathBuf> for RootPath2 {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}


impl AsRef<PathBuf> for RootPath2 {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

impl AsRef<Path> for RootPath2 {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}


impl RootPath2 {

    pub fn file_tree_path(&self) -> PathBuf {
        let mut p = self.0.clone();
        p.push(FILES_DIR_NAME);
        p
    }

    pub fn sqlite_path(&self) -> PathBuf {
        let mut p = self.0.clone();
        p.push(SQLITE_DB_NAME);
        p
    }
}
