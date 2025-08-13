use std::env;

use vs_cmdline::{Result, Error, Command, Response};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let result = Command::parse(&args[1..])
        .map_err(Error::Usage)
        .and_then(|c| c.exec())
        .inspect(|r: &Response| println!("{}", r));

    if let Err(e) = result {
        println!("ERROR: {}", e);
    }

    Ok(())
}
