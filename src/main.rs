use parsed::SWCParser;
use std::error::Error;
mod parsed;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/data/test.ts";
    let parser = SWCParser::new(path)?;

    dbg!(parser.module);

    Ok(())
}
