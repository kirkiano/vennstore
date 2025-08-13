
use util::{Has, Filename, ProperPathBuf};
use crate::{tree, Tree, sqlite,
            sqlite_tree::{Error, Result, SqliteFileTree},
            traits::{Delete, Exists, Insert, Remove}};



impl Insert<ProperPathBuf, Filename, Error> for SqliteFileTree {
    fn insert(&self, from: &ProperPathBuf) -> Result<Filename> {
        let ftp: Tree = self.get();

        // file tree operation first, so that, if it fails,
        // sqlite will remain in a consistent state
        let f = ftp.insert(from).map_err(Error::Tree)?;
        self.sqlite.insert(&f).map_err(Error::Sqlite)
    }
}


impl Remove<Filename, ProperPathBuf, Error> for SqliteFileTree {
    fn remove(&self, f: &Filename, dest: &ProperPathBuf) -> Result<()> {
        let ftp: Tree = self.get();

        // file tree operation first, so that, if it fails,
        // sqlite will remain in a consistent state
        ftp.remove(f, dest).map_err(Error::Tree)?;
        self.sqlite.delete(f).map_err(Error::Sqlite)
    }
}


impl Exists<Filename, Error> for SqliteFileTree {

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::Tree(tree::Error::AlreadyExists(_))) ||
        matches!(e, Error::Sqlite(sqlite::Error::FileAlreadyExists(_)))
    }

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::Tree(tree::Error::NotFound(_))) ||
        matches!(e, Error::Sqlite(sqlite::Error::FileNotFound(_)))
    }

    fn exists(&self, f: &Filename) -> Result<bool> {
        let ftp: Tree = self.get();
        if !ftp.exists(f).map_err(Error::Tree)? { return Ok(false) }
        self.sqlite.exists(f).map_err(Error::Sqlite)
    }
}
