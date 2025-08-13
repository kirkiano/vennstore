use std::{fmt, path::Path};

use util::{Filename, ProperPathBuf};
use crate::{FILES_DIR_NAME, SQLITE_DB_NAME};



/// The root of the file store
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootPath(ProperPathBuf);


impl fmt::Display for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p: &Path = self.0.as_ref();
        p.display().fmt(f)
    }
}


impl From<ProperPathBuf> for RootPath {
    fn from(p: ProperPathBuf) -> Self {
        RootPath(p)
    }
}


impl AsRef<ProperPathBuf> for RootPath {
    fn as_ref(&self) -> &ProperPathBuf {
        &self.0
    }
}

impl AsRef<Path> for RootPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl RootPath {

    pub fn file_tree_path(&self) -> ProperPathBuf {
        self.0.clone() + &Filename::from(FILES_DIR_NAME)
    }

    pub fn sqlite_path(&self) -> ProperPathBuf {
        self.0.clone() + &Filename::from(SQLITE_DB_NAME)
    }
}
