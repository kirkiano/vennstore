
use util::Filename;
use crate::{Tag, Set,
            traits::{Insert, Lookup, Delete},
            sqlite_tree::{Result, Error, SqliteFileTree}};



impl Insert<(Filename, Tag), (), Error> for SqliteFileTree {
    fn insert(&self, ft: &(Filename, Tag)) -> Result<()> {
        self.sqlite.insert(ft).map_err(Error::Sqlite)
    }
}


impl Delete<(Filename, Tag), Error> for SqliteFileTree {
    fn delete(&self, ft: &(Filename, Tag)) -> Result<()> {
        self.sqlite.delete(ft).map_err(Error::Sqlite)
    }
}


impl Lookup<Filename, Set<Tag>, Error> for SqliteFileTree {
    fn lookup(&self, f: &Filename) -> Result<Set<Tag>> {
        self.sqlite.lookup(f).map_err(Error::Sqlite)
    }
}


impl Lookup<Tag, Set<Filename>, Error> for SqliteFileTree {
    fn lookup(&self, t: &Tag) -> Result<Set<Filename>> {
        self.sqlite.lookup(t).map_err(Error::Sqlite)
    }
}
