
use thiserror::Error;

use util::Filename;
use crate::Tag;



#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("initialization: {0}")]
    Init(String),

    #[error("preexisting filename {0}")]
    FileAlreadyExists(Filename),

    #[error("no such filename {0}")]
    FileNotFound(Filename),

    #[error("preexisting tag {0}")]
    TagAlreadyExists(Tag),

    #[error("no tag {0}")]
    NoSuchTag(Tag),

    #[error("{0} already tags {1}")]
    AlreadyTagged(Tag, Filename),

    #[error("{0} does not tag {1}")]
    NotTagged(Tag, Filename),

    #[error("internal: {0}")]
    Internal(String),
}


impl Error {

    pub fn init<E>(err: E) -> Self
    where E: ToString,
    {
        Self::Init(err.to_string())
    }

    pub fn internal<E>(err: E) -> Self
    where E: ToString,
    {
        Self::Internal(err.to_string())
    }
}
