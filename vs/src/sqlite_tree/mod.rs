
mod error;
mod struc;
mod impls;

#[cfg(test)]
mod tests;


pub use error::Error;
pub use struc::SqliteFileTree;


type Result<T> = std::result::Result<T, Error>;