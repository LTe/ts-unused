use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use swc_common::{SourceFile, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub struct SWCParser<'a> {
    parser: Parser<Lexer<'a>>
}

impl<'a> SWCParser<'a> {
    pub fn new(path: &'a str) -> Result<SWCParser<'a>, Box<dyn Error>> {
        let source_file = source_file(path)?;
        let parser = parser(source_file);

        Ok(Self { parser })
    }

    pub fn parse(&mut self) -> Module {
        let module = self.parser
            .parse_typescript_module()
            .expect("Failed to parse module.");

        module
    }
}

fn source_file(path: &str) -> Result<StringInput, Box<dyn Error>> {
    let source_map: SourceMap = Default::default();
    let source_file = source_map.load_file(Path::new(path))?;

    Ok(StringInput::from(&*Rc::clone(&source_file)))
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
