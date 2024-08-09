use std::{fmt, path::{Path, PathBuf}};

use crate::{constants::TAGS_DIR, ensure_dir_created};


#[derive(Debug, Clone)]
pub struct Tag(String);


impl From<String> for Tag {
    fn from(value: String) -> Self {
        Self(value.to_lowercase())
    }
}

impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        value.0
    }
}


impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}


impl Tag {
    pub fn path(&self) -> PathBuf {
        Path::new(TAGS_DIR)
            .join(self.0.clone())
    }

    pub fn ensure_created(&self) {
        ensure_dir_created(self.path())
    }
}
