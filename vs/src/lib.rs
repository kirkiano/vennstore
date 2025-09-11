/*!

This crate provides Vennstore's core functionality --- moving files from
a given location into the store, and tagging it.

*/

mod constants;
mod error;
mod hash;
mod tag;
pub mod staging;
pub mod tree;
pub mod tree2;
pub mod traits;
mod root;
mod sqlite;
mod sqlite_tree;

#[cfg(test)]
mod test_utils;


pub use constants::*;
pub use error::Error;
pub use hash::Hash;
pub use tag::Tag;
pub use staging::Staging;
pub use tree::Tree;
pub use tree2::Tree2;
pub use root::RootPath;
pub use sqlite_tree::SqliteFileTree;
pub use traits::Store;

pub type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////

use util::Filename;

pub type Set<T> = std::collections::HashSet<T>;
type FT = (Filename, Tag);


pub type SFT = SqliteFileTree;
