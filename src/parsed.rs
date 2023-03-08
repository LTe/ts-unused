use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use swc_common::{SourceFile, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub struct SWCParser {
    pub module: Module,
}

impl SWCParser {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let source_file = source_file(path)?;
        let string_input = StringInput::from(&*source_file);
        let mut parser = parser(string_input.clone());
        let module = parser
            .parse_typescript_module()
            .expect("Invalid typescript file");

        Ok(SWCParser { module })
    }
}

fn source_file(path: &str) -> Result<Rc<SourceFile>, Box<dyn Error>> {
    let source_map: SourceMap = Default::default();
    let source_file = source_map.load_file(Path::new(path))?;

    Ok(source_file)
}

fn parser(source_file: StringInput) -> swc_ecma_parser::Parser<Lexer> {
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        source_file,
        None,
    );

    Parser::new_from(lexer)
}
