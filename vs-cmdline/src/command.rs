use std::path::PathBuf;

use clap::Subcommand;



#[derive(Debug, Clone, Subcommand)]
pub enum Command {

    Insert {
        file: PathBuf,
    },

    Tag {
        file: PathBuf,
        tag: String,
    },

    Tags {
        file:PathBuf,
    },

    Find {
        tag: String,
    },

    Remove {
        file: PathBuf,
        dest: PathBuf,
    }
}
