use std::env;

use vennstore::{Result, Error, Command, Response, exec};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let result = Command::parse(&args[1..])
        .map_err(Error::Usage)
        .and_then(|c| exec(&c))
        .inspect(|r: &Response| println!("{}", r));

    if let Err(e) = result {
        println!("ERROR: {}", e);
    }

    Ok(())
}
