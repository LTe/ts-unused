use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use swc_common::{SourceFile, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

#[derive(Debug)]
pub struct UnusedParser {
    pub module: Module,
}

impl UnusedParser {
    pub fn new(path: &str) -> UnusedParser {
        let module = parse(path);

        UnusedParser {
            module: module.unwrap(),
        }
    }
}

fn parse(path: &str) -> Result<Module, Box<dyn Error>> {
    let source_file = source_file(path)?;
    let string_input = StringInput::from(&*source_file);
    let parser = parser(string_input)?;

    Ok(parser)
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
