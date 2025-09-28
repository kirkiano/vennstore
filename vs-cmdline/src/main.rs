use clap::Parser;

use vs::{Result, RootPath2, SFT2};
use vs_cmdline::{Args, exec};



// TODO: consider catching CTRL-C, which might otherwise leave the
// system in an inconsistent state (ie, between db and file tree)
fn main() -> Result<()> {
    let cargs = Args::parse();
    let cmd = cargs.command;
    println!("root: {}", cargs.root.display());
    println!("main: {:?}", cmd);
    let root = RootPath2::from(cargs.root);
    let sft = SFT2::create_idempotently(root).unwrap();
    if let Err(e) = exec(&sft, cmd) { println!("{}", e); }
    Ok(())
}
