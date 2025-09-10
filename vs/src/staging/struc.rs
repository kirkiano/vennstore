use std::{fmt,
          path::{Path, PathBuf}};


#[derive(Debug, Clone)]
pub struct Staging(PathBuf);


impl fmt::Display for Staging {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "staging area at {}", self.0.display())
    }
}


impl From<PathBuf> for Staging {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}


impl AsRef<Path> for Staging {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
