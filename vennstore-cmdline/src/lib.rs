
mod util;
mod constants;
mod filename;
mod tag;
mod api;
mod algebra;
pub mod command;
mod error;

pub use util::ensure_dir_created;
pub use constants::*;
pub use filename::Filename;
pub use tag::Tag;
pub use api::*;
pub use algebra::Expr;
pub use command::Command;
pub use error::Error;


pub type Result<T> = std::result::Result<T, Error>;
