use crate::parser::SWCParser;
use crate::visitor::{TypescriptType, Visitor};
use swc_ecma_visit::Visit;

pub struct Checker {
  visitor: Visitor,
}

impl Checker {
  pub fn check(path: &str) -> Self {
    let parser = SWCParser::new(path).unwrap();

    let mut visitor = Visitor::new();

    visitor.visit_module(&parser.module);

    Checker { visitor }
  }

  pub fn typescript_types(self) -> Vec<TypescriptType> {
    self.visitor.typescript_types()
  }
}
