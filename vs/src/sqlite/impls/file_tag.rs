
use rusqlite::{self, Connection, params, ErrorCode};

use util::Filename;
use crate::{Set, Tag, FT,
            traits::{Insert, Lookup, Exists, AssertExists, Delete},
            sqlite::{Result, Error}};



impl Insert<FT, (), Error> for Connection {
    fn insert(&self, (f, t): &FT) -> Result<()> {
        if !self.exists(f)? { Err(Error::FileNotFound(f.clone()))? }
        if !self.exists(t)? { self.insert(t)?; }
        let sql = "INSERT INTO file_tag (file_id, tag_id)
                   SELECT F.id, T.id
                   FROM file F, tag T
                   WHERE F.name = ?1 AND T.name = ?2";
        self.execute(sql, (&f, t))
            .map_err(|e| match e.sqlite_error_code() {
                Some(ErrorCode::ConstraintViolation) =>
                    Error::AlreadyTagged(t.clone(), f.clone()),
                _ => Error::internal(e),
            })?;
        Ok(())
    }
}


impl Exists<FT, Error> for Connection {

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::NotTagged(_, _))
    }

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::AlreadyTagged(_, _))
    }

    fn exists(&self, (f, t): &FT) -> Result<bool> {
        let sql = "SELECT 1 FROM file_tag
                   WHERE file_id in (SELECT id FROM file WHERE name = ?1)
                     AND tag_id in (SELECT id FROM tag WHERE name = ?2)";
        match self.query_one(sql, (f, t), |_row| Ok(())) {
            Ok(())                                    => Ok(true),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(Error::internal(e)),
        }
    }
}


impl Delete<FT, Error> for Connection {
    fn delete(&self, (f, t): &FT) -> Result<()> {
        self.assert_exists(f)?;
        self.assert_exists(t)?;
        let sql = "DELETE FROM file_tag
                   WHERE file_id in (SELECT id FROM file WHERE name = ?1)
                     AND tag_id in (SELECT id FROM tag WHERE name = ?2)";
        self.execute(sql, (f, t))
            .map_err(Error::internal)
            .and_then(|n: usize| match n {
                0 => Err(Error::NotTagged(t.clone(), f.clone())),
                1 => Ok(()),
                _ => {
                    let msg = format!("{t} tags {f} more than once");
                    Err(Error::internal(msg))
                },
            })
            .map_err(Error::internal)?;
        Ok(())
    }
}


impl Lookup<Filename, Set<Tag>, Error> for Connection {
    fn lookup(&self, f: &Filename) -> Result<Set<Tag>> {
        self.assert_exists(f)?;
        let sql = "SELECT T.name
                   FROM tag T
                   JOIN file_tag FT on T.id = FT.tag_id
                   JOIN file F on F.id = FT.file_id
                   WHERE F.name = ?1";
        let mut stmt = self.prepare(sql).map_err(Error::internal)?;
        let mut rows = stmt.query(params![&f]).map_err(Error::internal)?;
        let mut tags = Set::new();
        while let Some(row) = rows.next().map_err(Error::internal)? {
            tags.insert(row.get(0).map_err(Error::internal)?);
        }
        Ok(tags)
    }
}


impl Lookup<Tag, Set<Filename>, Error> for Connection {
    fn lookup(&self, t: &Tag) -> Result<Set<Filename>> {
        self.assert_exists(t)?;
        let sql = "SELECT F.name
                   FROM file F
                   JOIN file_tag FT on F.id = FT.file_id
                   JOIN tag T on T.id = FT.tag_id
                   WHERE T.name = ?1";
        let mut stmt = self.prepare(sql).map_err(Error::internal)?;
        let mut rows = stmt.query(params![t]).map_err(Error::internal)?;
        let mut fs = Set::new();
        while let Some(row) = rows.next().map_err(Error::internal)? {
            fs.insert(row.get(0).map_err(Error::internal)?);
        }
        Ok(fs)
    }

}
