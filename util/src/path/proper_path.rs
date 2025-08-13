use std::{fmt,
          path::Path};

use crate::Filename;



#[derive(Debug, Clone)]
pub struct ProperPath<'a> {
    pub full: &'a Path,
    pub fname: Filename,
}

impl fmt::Display for ProperPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.full.display().fmt(f)
    }
}

impl<'a> TryFrom<&'a Path> for ProperPath<'a> {
    type Error = ();

    fn try_from(full: &'a Path) -> Result<Self, Self::Error> {
        Filename::try_from(full).map(|fname| Self { full, fname })
    }
}

impl AsRef<Path> for ProperPath<'_> {
    fn as_ref(&self) -> &Path {
        &self.full
    }
}

impl AsRef<Filename> for ProperPath<'_> {
    fn as_ref(&self) -> &Filename {
        &self.fname
    }
}
