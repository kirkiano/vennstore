use std::{io, path::{Path, PathBuf}};

use thiserror::Error;

use util::Filename;



#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {

    #[error("already exists: {0}")]
    AlreadyExists(Filename),

    #[error("not found: {0}")]
    NotFound(Filename),

    #[error("not a directory: {0}")]
    NotADir(PathBuf),

    #[error("cannot move {0} to {1}: {2}")]
    CannotMove(PathBuf, PathBuf, String),

    #[error("internal: {0}")]
    Internal(String),
}


impl Error {

    pub fn not_a_dir<P: AsRef<Path>>(p: P) -> Self {
        let pb: PathBuf = p.as_ref().to_owned();
        Self::NotADir(pb)
    }

    pub fn cannot_move<F, T>(from: F, to: T) -> impl FnOnce(io::Error) -> Self
    where F: AsRef<Path>,
          T: AsRef<Path>,
    {
        let f = PathBuf::from(from.as_ref());
        let t = PathBuf::from(to.as_ref());
        move |e| Error::CannotMove(f, t, e.to_string())
    }

    pub fn internal<E>(err: E) -> Self
    where E: std::error::Error,
    {
        Error::Internal(err.to_string())
    }
}
