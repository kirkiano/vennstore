use std::path::PathBuf;

use thiserror::Error;

use util::Filename;
use crate::{sqlite, tree};



#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {

    #[error("db: {0}")]
    Sqlite(sqlite::Error),

    #[error("file tree: {0}")]
    Tree(tree::Error),

    #[error("db and file tree disagree about existence of {0}")]
    Discrepancy(Filename),

    #[error("{0} is not a proper filepath")]
    NotProperFilePath(PathBuf),

    #[error("internal: {0}")]
    Internal(String),
}


impl Error {

    pub fn internal<E>(err: E) -> Self
    where E: ToString,
    {
        Self::Internal(err.to_string())
    }
}
