use std::io;

use thiserror::Error;

use crate::Filename;


#[derive(Debug, Error)]
pub enum Error {
    #[error("no file called {0}")]
    NoSuchFilename(Filename),

    #[error("I/O: {0}")]
    IO(io::Error),

    #[error("internal: {0}")]
    Internal(String),
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}


impl Error {
    pub fn internal<S>(s: S) -> Self
    where S: ToString
    {
        Self::Internal(s.to_string())
    }
}