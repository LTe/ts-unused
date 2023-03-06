use std::path::Path;
use swc_common::{SourceMap, SourceFile};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use std::error::Error;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/data/test.ts";

    parse(path)?;

    Ok(())
}

fn parse(path: &str) -> Result<(), Box<dyn Error>> {
    let source_file = source_file(path)?;
    let string_input = StringInput::from(&*source_file);
    let parser = parser(string_input);

    println!("{:?}", parser);

    Ok(())
}

fn source_file(path: &str) -> Result<Rc<SourceFile>, Box<dyn Error>> {
    let source_map: SourceMap = Default::default();
    let source_file = source_map.load_file(Path::new(path))?;

    Ok(source_file)
}

fn parser(source_file: StringInput) -> Result<swc_ecma_ast::Module, Box<dyn Error>> {
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        source_file,
        None,
        );

    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_typescript_module()
        .expect("Failed to parse module.");

    Ok(module)
}
