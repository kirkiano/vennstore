use std::fmt;

use crate::{Tag, Filename};


#[derive(Debug)]
pub enum Response {
    Null,
    FileAdded(Filename),
    FileTagged(Filename, Tag),
}


impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Null =>
                write!(f, ""),

            Self::FileAdded(fname) =>
                write!(f, "Added {}", fname.hashed_path().to_string_lossy()),

            Self::FileTagged(fname, t) =>
                write!(f, "Put tag {} on {}",
                       t, fname.hashed_path().to_string_lossy()),
        }
    }
}
