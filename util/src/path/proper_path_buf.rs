use std::{fmt,
          ops::Add,
          path::{Path, PathBuf},
          ffi::{OsStr, OsString}};

use crate::Filename;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProperPathBuf {
    full: PathBuf,
    fname: Filename,
}

impl fmt::Display for ProperPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.full.display().fmt(f)
    }
}


impl From<Filename> for ProperPathBuf {
    fn from(fname: Filename) -> Self {
        let oss: &OsStr = fname.as_ref();
        let full = PathBuf::from(oss);
        Self { full, fname }
    }
}

impl TryFrom<PathBuf> for ProperPathBuf {
    type Error = ();

    fn try_from(full: PathBuf) -> Result<Self, Self::Error> {
        let p: &Path = full.as_ref();
        Filename::try_from(p).map(|fname| Self { full, fname })
    }
}

impl TryFrom<OsString> for ProperPathBuf {
    type Error = <Self as TryFrom<PathBuf>>::Error;

    fn try_from(s: OsString) -> Result<Self, Self::Error> {
        let p = PathBuf::from(s);
        p.try_into()
    }
}

impl TryFrom<String> for ProperPathBuf {
    type Error = <Self as TryFrom<PathBuf>>::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let p = PathBuf::from(s);
        p.try_into()
    }
}

impl TryFrom<&str> for ProperPathBuf {
    type Error = <Self as TryFrom<PathBuf>>::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_owned())
    }
}

impl AsRef<OsStr> for ProperPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.full.as_ref()
    }
}

impl AsRef<Path> for ProperPathBuf {
    fn as_ref(&self) -> &Path {
        self.full.as_ref()
    }
}

impl AsRef<PathBuf> for ProperPathBuf {
    fn as_ref(&self) -> &PathBuf {
        &self.full
    }
}

impl AsRef<Filename> for ProperPathBuf {
    fn as_ref(&self) -> &Filename {
        &self.fname
    }
}

impl ProperPathBuf {
    // TODO: shouldn't this return non-Option &Path?
    pub fn parent(&self) -> &Path {
        self.full.parent().unwrap()
    }
}

impl Add<&Self> for ProperPathBuf {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let mut full = self.full;
        let s: &OsStr = other.as_ref();
        full.push(s);
        Self { full, fname: other.fname.clone() }
    }
}

impl Add<&Filename> for ProperPathBuf {
    type Output = Self;

    fn add(self, fname: &Filename) -> Self::Output {
        let mut full = self.full;
        let s: &OsStr = fname.as_ref();
        full.push(s);
        Self { full, fname: fname.clone() }
    }
}
