
use rusqlite::{Connection, ErrorCode};

use crate::{Tag,
            traits::{Insert, Exists, AssertExists, Delete},
            sqlite::{Result, Error}};



impl Insert<Tag, Tag, Error> for Connection {
    fn insert(&self, t: &Tag) -> Result<Tag> {
        self.execute("INSERT INTO tag (name) VALUES (?1)", (&t,))
            .map(|_: usize| t.clone())
            .map_err(|e| match e.sqlite_error_code() {
                Some(ErrorCode::ConstraintViolation) =>
                    Error::TagAlreadyExists(t.clone()),
                _ => Error::internal(e)
            })
    }
}


impl Delete<Tag, Error> for Connection {
    fn delete(&self, t: &Tag) -> Result<()> {
        self.execute("DELETE FROM tag WHERE name = ?1", (&t,))
            .map_err(Error::internal)
            .and_then(|n: usize| match n {
                0 => Err(Error::NoSuchTag(t.clone())),
                1 => Ok(()),
                _ => {
                    let msg = format!("{} rows of tag '{}' deleted", n, t);
                    Err(Error::internal(msg))
                },
            })

    }
}


impl Exists<Tag, Error> for Connection {

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::TagAlreadyExists(_))
    }

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::NoSuchTag(_))
    }

    fn exists(&self, t: &Tag) -> Result<bool> {
        let sql = "SELECT 1 FROM tag WHERE name = ?1";
        match self.query_one(sql, (&t,), |_row| Ok(true)) {
            Ok(b) =>
                Ok(b),

            Err(rusqlite::Error::QueryReturnedNoRows) =>
                Ok(false),

            Err(e) =>
                Err(Error::internal(e)),
        }
    }
}


impl AssertExists<Tag, Error> for Connection {
    fn assert_exists(&self, t: &Tag) -> Result<()> {
        if !self.exists(t)? { Err(Error::NoSuchTag(t.clone())) }
        else { Ok(()) }
    }

    fn assert_does_not_exist(&self, t: &Tag) -> Result<()> {
        if self.exists(t)? { Err(Error::TagAlreadyExists(t.clone())) }
        else { Ok(()) }
    }
}
