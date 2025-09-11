use std::io;

use thiserror::Error;

use crate::Filename;


#[derive(Debug, Error)]
pub enum Error {
    #[error("no file called {0}")]
    NoSuchFilename(Filename),

    #[error("I/O: {0}")]
    IO(io::Error),
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}