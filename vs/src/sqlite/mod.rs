
mod error;
mod utils;
mod impls;
mod open;

#[cfg(test)]
mod tests;

pub use error::Error;
pub use open::create_idempotently;
use utils::{is_uniqueness_violation, get_file_id};


pub type Result<T> = std::result::Result<T, Error>;


type FileId = i64;
