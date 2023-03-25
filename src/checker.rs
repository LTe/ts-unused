use crate::parser::SWCParser;
use crate::visitor::{TypescriptType, Visitor};
use stc_ts_builtin_types::Lib;
use stc_ts_env::{Env, ModuleConfig, Rule};
use stc_ts_file_analyzer::env::EnvFactory;
use stc_ts_module_loader::resolvers::node::NodeResolver;
use stc_ts_type_checker::{
  loader::{DefaultFileLoader, ModuleLoader},
  Checker,
};
use swc_ecma_ast::EsVersion;

use std::sync::Arc;
use swc_common::errors::{ColorConfig, EmitterWriter, Handler};
use swc_common::{Globals, SourceMap};
use swc_ecma_visit::Visit;

#[derive(Debug)]
pub struct UnusedChecker {
  visitor: Visitor,
}

impl UnusedChecker {
  pub fn check(path: &str) -> Self {
    let parser = SWCParser::new(path).unwrap();

    let mut visitor = Visitor::new();
    visitor.visit_module(&parser.module);

    UnusedChecker { visitor }
  }

  pub fn typescript_types(self) -> Vec<TypescriptType> {
    self.visitor.typescript_types()
  }

  pub fn create_stc_checker(
    &self,
    path: &str,
  ) -> Checker<ModuleLoader<DefaultFileLoader, NodeResolver>> {
    let checker = swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
      let cm = Arc::new(SourceMap::default());
      let handler = {
        let emitter = Box::new(EmitterWriter::stderr(
          ColorConfig::Always,
          Some(cm.clone()),
          false,
          false,
        ));
        Arc::new(Handler::with_emitter(true, false, emitter))
      };

      let mut libs = Lib::load("es5");
      libs.sort();
      libs.dedup();

      let env = Env::simple(
        Rule {
          ..Default::default()
        },
        EsVersion::latest(),
        ModuleConfig::None,
        &libs,
      );

      println!("Przed checker!");

      let checker = Checker::new(
        cm.clone(),
        handler.clone(),
        env.clone(),
        None,
        ModuleLoader::new(cm, env, NodeResolver, DefaultFileLoader),
      );

      checker
    });

    checker
  }
}
