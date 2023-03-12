use std::error::Error;
use swc_ecma_visit::Visit;
use ts_unused::parser::SWCParser;
use ts_unused::types::Visitor;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./tests/data/test.ts";
    let parser = SWCParser::new(path)?;
    let module = parser.module;

    let mut visitor = Visitor::new();

    visitor.visit_module(&module);

    dbg!(visitor);

    Ok(())
}
