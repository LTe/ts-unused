use std::path::Path;
use std::{error::Error, sync::Arc};
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

fn source_file(path: &str) -> Result<Arc<SourceFile>, Box<dyn Error>> {
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

#[cfg(test)]
mod test {
  use super::SWCParser;

  #[test]
  fn it_creates_parsed_modules() {
    let parser = SWCParser::new("./tests/data/test.ts");

    assert!(parser.is_ok())
  }

  #[test]
  fn it_does_not_create_parsed_module_for_non_existing_file() {
    let parser = SWCParser::new("./tests/data/__DO_NOT_EXISTS__.ts");

    assert!(parser.is_err())
  }
}
