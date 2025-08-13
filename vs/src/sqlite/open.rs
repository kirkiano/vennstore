use std::path::Path;

use rusqlite::Connection;

use super::{Result, Error};



// idempotently creates and initializes a fresh db
pub fn create_idempotently<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let db = Connection::open(path.as_ref()).map_err(Error::internal)?;
    init(&db).map_err(Error::internal)?;
    Ok(db)
}


fn init(db: &Connection) -> rusqlite::Result<()> {
    create_file_table(db)?;
    create_tag_table(db)?;
    create_file_tag_table(db)?;
    Ok(())
}


fn create_file_table(db: &Connection) -> rusqlite::Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS file (
                   id INTEGER PRIMARY KEY AUTOINCREMENT,
                   name VARCHAR(256) UNIQUE NOT NULL
               )";
    db.execute(sql, ())?;
    Ok(())
}


fn create_tag_table(db: &Connection) -> rusqlite::Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS tag (
                   id INTEGER PRIMARY KEY AUTOINCREMENT,
                   name VARCHAR(256) UNIQUE
               )";
    db.execute(sql, ())?;
    Ok(())
}


fn create_file_tag_table(db: &Connection) -> rusqlite::Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS file_tag (
                   file_id INTEGER NOT NULL,
                   tag_id INTEGER NOT NULL,
                   FOREIGN KEY(file_id) REFERENCES file(id),
                   FOREIGN KEY(tag_id) REFERENCES tag(id),
                   UNIQUE (file_id, tag_id)
               )";
    db.execute(sql, ())?;
    Ok(())
}
