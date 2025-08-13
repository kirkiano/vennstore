use std::fmt;

use rusqlite::{Result, ToSql,
               types::{FromSql, ToSqlOutput, ValueRef, FromSqlResult}};

#[cfg(test)]
use util::time::now_nanos;

#[cfg(test)]
use crate::test_utils::Mock;



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String);


impl From<String> for Tag {
    fn from(value: String) -> Self {
        Self(value.to_lowercase())
    }
}

impl From<Tag> for String {
    fn from(t: Tag) -> Self {
        t.0
    }
}


impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}


impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/////////////////////////////////////////////////////////////////////
/// sqlite

impl ToSql for Tag {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        self.as_ref().to_sql()
    }
}

impl FromSql for Tag {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        let n: String = FromSql::column_result(v)?;
        Ok(n.into())
    }
}

/////////////////////////////////////////////////////////////////////
/// mock

#[cfg(test)]
impl Mock<()> for Tag {
    fn mock(_: ()) -> Tag {
        format!("dummy_tag_{}", now_nanos()).into()
    }
}
