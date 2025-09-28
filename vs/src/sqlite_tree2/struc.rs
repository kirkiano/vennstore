use std::{fmt, fs};

use util::Has;
use crate::{Result, Error, Tree2, RootPath2, sqlite};


use rusqlite::Connection;


#[derive(Debug)]
pub struct SqliteFileTree2 {
    pub(super) root: RootPath2,
    pub(super) sqlite: Connection,
}


impl fmt::Display for SqliteFileTree2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sqlite-file-tree2 at {}", self.root)
    }
}



impl SqliteFileTree2 {

    pub fn create_idempotently(root: RootPath2) -> Result<Self> {
        fs::create_dir_all(root.file_tree_path()).map_err(Error::internal)?;
        let sqlite = sqlite::create_idempotently(root.sqlite_path()).map_err(Error::internal)?;
        let s = Self { root, sqlite };
        Ok(s)
    }

}


impl Has<Tree2> for SqliteFileTree2 {
    fn get(&self) -> Tree2 {
        Tree2::from(self.root.file_tree_path())
    }
}
