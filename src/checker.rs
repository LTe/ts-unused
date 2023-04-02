use crate::parser::SWCParser;
use crate::visitor::{TypescriptType, Visitor};
use stc_ts_builtin_types::Lib;
use stc_ts_env::{Env, JsxMode, ModuleConfig, Rule};
use stc_ts_file_analyzer::analyzer::Analyzer;
use stc_ts_file_analyzer::env::EnvFactory;
use stc_ts_module_loader::resolvers::node::NodeResolver;
use stc_ts_type_checker::loader::{LoadModule, ModuleRecord};
use stc_ts_type_checker::{
  loader::{DefaultFileLoader, ModuleLoader},
  Checker,
};
use stc_ts_types::Type;
use std::path::PathBuf;
use std::sync::Arc;
use swc_common::errors::{ColorConfig, EmitterWriter, Handler};
use swc_common::{FileName, Globals, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_visit::Visit;

#[derive(Debug)]
pub struct UnusedChecker {
  visitor: Visitor,
}

pub struct NewUnusedChecker {
  typescript_types: Vec<stc_ts_types::Id>,
}

impl NewUnusedChecker {
  pub fn new() -> Self {
    NewUnusedChecker {
      typescript_types: vec![],
    }
  }

  pub fn typescript_types(self) -> Vec<stc_ts_types::Id> {
    self.typescript_types
  }

  pub fn check(path: &str) -> Self {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
      let checker = Self::create_stc_checker();

      let path_buf = PathBuf::from(path);
      let arc = Arc::new(FileName::Real(path_buf));
      let module_id = checker.check(arc.clone());

      let types_for_module = checker.get_types(module_id);

      let private_types = match types_for_module.clone() {
        Some(types) => match types.normalize() {
          Type::Module(type_module) => Some(type_module.exports.private_types.clone()),
          _ => None,
        },

        None => None,
      };

      let private_vars = match types_for_module.clone() {
        Some(types) => match types.normalize() {
          Type::Module(type_module) => Some(type_module.exports.private_vars.clone()),
          _ => None,
        },

        None => None,
      };

      dbg!(private_types.clone());
      dbg!(private_vars.clone());

      match private_types {
        Some(types) => NewUnusedChecker {
          typescript_types: types.into_keys().collect(),
        },
        None => NewUnusedChecker {
          typescript_types: vec![],
        },
      }
    })
  }

  pub fn create_stc_checker() -> Checker<ModuleLoader<DefaultFileLoader, NodeResolver>> {
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

    let checker = Checker::new(
      cm.clone(),
      handler.clone(),
      env.clone(),
      None,
      ModuleLoader::new(cm, env, NodeResolver, DefaultFileLoader),
    );

    checker
  }
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
}
