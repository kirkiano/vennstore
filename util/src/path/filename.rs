use std::{fmt,
          ffi::{OsStr, OsString},
          path::{Path, PathBuf}};

use rusqlite::{Result, ToSql,
               types::{FromSql, FromSqlError, FromSqlResult,
                       ToSqlOutput, Value, ValueRef}};


/// A filename, *ie*, an [`OsString`] that contains no slashes or
/// trailing double-dot.
///
/// This value is owned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filename(OsString);


impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_string_lossy().fmt(f)
    }
}

impl From<&str> for Filename {
    fn from(s: &str) -> Self {
        Self(OsString::from(s))
    }
}

impl TryFrom<&Path> for Filename {
    type Error = ();

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        p.file_name().ok_or(()).map(|s| Self(s.to_owned()))
    }
}

impl TryFrom<&PathBuf> for Filename {
    type Error = ();

    fn try_from(p: &PathBuf) -> Result<Self, Self::Error> {
        let p: &Path = p.as_ref();
        Self::try_from(p)
    }
}


impl AsRef<OsStr> for Filename {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl AsRef<OsString> for Filename {
    fn as_ref(&self) -> &OsString {
        &self.0
    }
}

impl AsRef<[u8]> for Filename {
    fn as_ref(&self) -> &[u8] {
        let s: &OsStr = self.as_ref();
        s.as_encoded_bytes()
    }
}

impl From<Filename> for PathBuf {
    fn from(f: Filename) -> Self {
        f.0.into()
    }
}


/////////////////////////////////////////////////////////////////////
/// sqlite

impl ToSql for Filename {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let v = Value::Text(self.to_string());
        Ok(ToSqlOutput::Owned(v))
    }
}

impl FromSql for Filename {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        let s: String = FromSql::column_result(v)?;
        Self::try_from(&PathBuf::from(s))
            .map_err(|_| FromSqlError::InvalidType)
    }
}
