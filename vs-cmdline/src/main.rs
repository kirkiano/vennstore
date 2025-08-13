use clap::Parser;

use util::ProperPathBuf;
use vs::{Result, SFT};
use vs_cmdline::{Args, exec};



// TODO: consider catching CTRL-C, which might otherwise leave the
// system in an inconsistent state (ie, between db and file tree)
fn main() -> Result<()> {
    let cargs = Args::parse();
    let cmd = cargs.command;
    println!("root: {}", cargs.root.display());
    println!("main: {:?}", cmd);
    let root = ProperPathBuf::try_from(cargs.root).unwrap();
    let sft = SFT::create_idempotently(root).unwrap();
    if let Err(e) = exec(&sft, cmd) { println!("{}", e); }
    Ok(())
}
