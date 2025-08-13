use rusqlite::{self, Connection, ErrorCode};

use util::Filename;
use crate::sqlite::{Result, Error, FileId};


pub fn is_uniqueness_violation(e: &rusqlite::Error) -> bool {
    matches!(e.sqlite_error_code(), Some(ErrorCode::ConstraintViolation))
}

fn is_no_rows(e: &rusqlite::Error) -> bool {
    matches!(e, rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_file_id(db: &Connection, f: &Filename) -> Result<FileId> {
    let sql = "SELECT id FROM file WHERE name = ?1";
    db.query_one(sql, (&f,), |row| row.get(0))
      .map_err(|e| {
          if is_no_rows(&e) { Error::FileNotFound(f.clone()) }
          else { Error::internal(e) }
      })
}
