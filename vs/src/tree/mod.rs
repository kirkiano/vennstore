
mod error;
mod struc;
mod impls;

#[cfg(test)]
mod tests;

pub use error::Error;
pub use struc::Tree;


pub type Result<T> = std::result::Result<T, Error>;
