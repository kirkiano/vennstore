use std::path::PathBuf;

use clap::Parser;

use crate::Command;



#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(short, long, value_name = "ROOT")]
    pub root: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}
