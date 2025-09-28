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
mod root2;
mod sqlite;
mod sqlite_tree;
mod sqlite_tree2;

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
pub use root2::RootPath2;
pub use sqlite_tree::SqliteFileTree;
pub use sqlite_tree2::SqliteFileTree2;
pub use traits::Store;

pub type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////

use util::Filename;

pub type Set<T> = std::collections::HashSet<T>;
type FT = (Filename, Tag);


pub type SFT = SqliteFileTree;
pub type SFT2 = SqliteFileTree2;
