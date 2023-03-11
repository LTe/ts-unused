mod parser;
mod types;

use parser::SWCParser;
use std::error::Error;
use swc_ecma_visit::Visit;
use types::TypescriptType;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/data/test.ts";
    let parser = SWCParser::new(path)?;
    let module = parser.module;

    let mut visitor = types::Visitor::new();

    visitor.visit_module(&module);

    dbg!(visitor);

    Ok(())
}
