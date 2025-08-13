
// mod error;
// mod api_eventually_to_be_deleted;
// mod algebra;
mod command;
mod cargs;
mod exec;


// pub use error::Error;
// pub use api_eventually_to_be_deleted::*;
// pub use algebra::Expr;

pub use command::Command;
pub use cargs::Args;
pub use exec::exec;
