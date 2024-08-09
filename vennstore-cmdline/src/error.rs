use std::{fmt, io, path::PathBuf};

use crate::{command, Filename};


#[derive(Debug)]
pub enum Error {
    /// Problem parsing command line args
    Usage(command::Error),

    /// The arg is the path of the file that should exist
    NoSuchSourceFile(String),

    /// The arg is the path of the file that should have been installed
    CannotInstallFile(String, io::Error),

    FileExists(Filename),

    NoSuchFile(Filename),

    CannotSymlink(PathBuf, PathBuf, io::Error),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Usage(e) =>
                write!(f, "Usage: {}", e),

            Self::NoSuchSourceFile(p) =>
                write!(f, "Could not find {}", p),

            Self::CannotInstallFile(p, e) =>
                    write!(f, "Cannot install {}: {}", p, e),

            Self::FileExists(fl) =>
                write!(f, "File {} already exists",
                       fl.hashed_path().to_string_lossy()),

            Self::NoSuchFile(fl) =>
                write!(f, "File {} does not exist", fl),

            Self::CannotSymlink(orig, link, err) =>
                write!(f, "Cannot symlink from {} to {}: {}",
                       link.to_string_lossy(),
                       orig.to_string_lossy(),
                       err),
        }
    }
}


impl std::error::Error for Error {}


impl Error {

    pub fn cannot_install_file(p: String) -> impl FnOnce(io::Error) -> Self {
        move |e: io::Error| Self::CannotInstallFile(p, e)
    }

    pub fn cannot_symlink(orig: PathBuf, link: PathBuf) ->
        impl FnOnce(io::Error) -> Self
    {
        move |e: io::Error| Self::CannotSymlink(orig, link, e)
    }

}
