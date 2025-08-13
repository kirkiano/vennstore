use std::fmt;

use crate::{Tag, Filename};
use super::Command;


pub type Result<T> = std::result::Result<T, Error>;


impl Command {
    pub fn parse(input: &[String]) -> Result<Self> {
        if input.len() == 0 {
            return Err(Error::Insufficient);
        }
        match input[0].as_str() {
            "init" => Ok(Self::Init),
            "add"  => Self::parse_add_file(&input[1..]),
            "tag"  => Self::parse_tag_file(&input[1..]),
            _      => Err(Error::UnknownCommand(input[0].clone())),
        }
    }

    fn parse_add_file(args: &[String]) -> Result<Self> {
        if args.len() == 0 { Err(Error::MissingFile) }
        else { Ok(Self::AddFile(args[0].clone())) }
    }

    fn parse_tag_file(args: &[String]) -> Result<Self> {
        if      args.len() < 1 { Err(Error::MissingTag) }
        else if args.len() < 2 { Err(Error::MissingFile) }
        else {
            let f = Filename::from(args[1].clone());
            let t = Tag::from(args[0].clone());
            Ok(Self::TagFile(f, t))
        }
    }
}


///////////////////////////////////////////////////////////
// parse error

#[derive(Debug)]
pub enum Error {
    Insufficient,
    UnknownCommand(String),
    MissingFile,
    MissingTag,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Insufficient =>
                write!(f, "No command given"),

            Self::UnknownCommand(c) =>
                write!(f, "Unknown command '{}'", c),

            Self::MissingFile =>
                write!(f, "Expected file as argument"),

            Self::MissingTag =>
                write!(f, "Expected tag as argument"),
        }
    }
}


impl std::error::Error for Error {}
