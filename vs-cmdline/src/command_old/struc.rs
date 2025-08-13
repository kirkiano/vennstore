use std::fmt;

use crate::{Tag, Filename};


#[derive(Debug, Clone)]
pub enum Command {

    /// Initialize the current directory into a VennStore
    Init,

    /// Add to the stash the file identified by the given path string
    AddFile(String),

    /// Associate the file with the given tag
    TagFile(Filename, Tag),
}


impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Init =>
                write!(f, "Initialize"),

            Self::AddFile(p) =>
                write!(f, "Add to store: {}", p),

            Self::TagFile(fl, t) =>
                write!(f, "Put tag '{}' on {}", t, fl),
        }
    }
}
