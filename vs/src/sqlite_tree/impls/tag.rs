
use crate::{Tag, sqlite,
            traits::{Insert, Exists, Delete},
            sqlite_tree::{Result, Error, SqliteFileTree}};


impl Insert<Tag, Tag, Error> for SqliteFileTree {
    fn insert(&self, t: &Tag) -> Result<Tag> {
        self.sqlite.insert(t).map_err(Error::Sqlite)
    }
}


impl Delete<Tag, Error> for SqliteFileTree {
    fn delete(&self, t: &Tag) -> Result<()> {
        self.sqlite.delete(t).map_err(Error::Sqlite)
    }
}


impl Exists<Tag, Error> for SqliteFileTree {

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::Sqlite(sqlite::Error::TagAlreadyExists(_)))
    }

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::Sqlite(sqlite::Error::NoSuchTag(_)))
    }

    fn exists(&self, t: &Tag) -> Result<bool> {
        self.sqlite.exists(t).map_err(Error::Sqlite)
    }
}
