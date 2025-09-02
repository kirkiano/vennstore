/*!

This crate provides Vennstore's core functionality --- moving files from
a given location into the store, and tagging it.

*/
mod constants;
mod hash;
mod tag;
pub mod tree;
pub mod traits;
mod root;
mod sqlite;
mod sqlite_tree;
mod vs;

#[cfg(test)]
mod test_utils;


pub use constants::*;
pub use hash::Hash;
pub use tag::Tag;
pub use tree::Tree;
pub use root::RootPath;
pub use sqlite_tree::{SqliteFileTree, Result, Error};
pub use traits::Store;
pub use vs::VSSqliteTree;


/////////////////////////////////////////////////////////////////////

use util::Filename;

pub type Set<T> = std::collections::HashSet<T>;
type FT = (Filename, Tag);


pub type SFT = SqliteFileTree;
