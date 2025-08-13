
use rusqlite::Connection;

use util::Filename;
use crate::{traits::{Insert, Exists, AssertExists, Delete},
            sqlite::{Result, Error, get_file_id, is_uniqueness_violation}};



impl Insert<Filename, Filename, Error> for Connection {
    fn insert(&self, f: &Filename) -> Result<Filename> {
        let sql = "INSERT INTO file (name) VALUES (?1)";
        self.execute(sql, (&f,))
            .map(|_| f.clone())
            .map_err(|e|
                if is_uniqueness_violation(&e) {
                    Error::FileAlreadyExists(f.clone())
                }
                else { Error::internal(e) })
        }
    }



impl Exists<Filename, Error> for Connection {

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::FileAlreadyExists(_))
    }

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::FileNotFound(_))
    }

    fn exists(&self, f: &Filename) -> Result<bool> {
        let sql = "SELECT 1 FROM file WHERE name = ?1";
        match self.query_one(sql, (&f,), |_row| Ok(true)) {
            Ok(b) =>
                Ok(b),

            Err(rusqlite::Error::QueryReturnedNoRows) =>
                Ok(false),

            Err(e) =>
                Err(Error::internal(e)),
        }
    }
}


impl AssertExists<Filename, Error> for Connection {
    fn assert_exists(&self, f: &Filename) -> Result<()> {
        if !self.exists(f)? { Err(Error::FileNotFound(f.clone())) }
        else { Ok(()) }
    }

    fn assert_does_not_exist(&self, f: &Filename) -> Result<()> {
        if self.exists(f)? { Err(Error::FileAlreadyExists(f.clone())) }
        else { Ok(()) }
    }
}


impl Delete<Filename, Error> for Connection {
    fn delete(&self, f: &Filename) -> Result<()> {
        let fid = get_file_id(self, f)?;
        let sql_tags = "DELETE FROM file_tag WHERE file_id = ?1";
        // TODO: detect whether a file was really deleted; throw error if not
        self.execute(sql_tags, (&fid,)).map_err(Error::internal)?;

        let sql_file = "DELETE FROM file WHERE id = ?1";
        self.execute(sql_file, (&fid,))
            .map(|nrows| nrows == 1)
            .map_err(Error::internal)?;
        Ok(())
    }
}
