use std::{fmt, fs};

use util::{Has, Filename, ProperPathBuf};
use crate::{Tree, RootPath, sqlite, FILES_DIR_NAME};
use super::{Result, Error};


use rusqlite::Connection;


#[derive(Debug)]
pub struct SqliteFileTree {
    pub(super) root: RootPath,
    pub(super) sqlite: Connection,
}


impl fmt::Display for SqliteFileTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sqlite-file-tree at {}", self.root)
    }
}



impl SqliteFileTree {

    pub fn create_idempotently(r: ProperPathBuf) -> Result<Self> {
        let root = RootPath::from(r);
        fs::create_dir_all(root.file_tree_path()).map_err(Error::internal)?;
        let sqlite = sqlite::create_idempotently(root.sqlite_path()).map_err(Error::internal)?;
        let s = Self { root, sqlite };
        Ok(s)
    }

}


impl Has<Tree> for SqliteFileTree {
    fn get(&self) -> Tree {
        let pp: &ProperPathBuf = self.root.as_ref();
        Tree::from(pp.clone() + &Filename::from(FILES_DIR_NAME))
    }
}
