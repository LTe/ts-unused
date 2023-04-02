use rnode::{NodeIdGenerator, RNode, VisitWith};
use stc_ts_builtin_types::Lib;
use stc_ts_env::{Env, JsxMode, ModuleConfig, Rule};
use stc_ts_file_analyzer::analyzer::Analyzer;
use stc_ts_file_analyzer::analyzer::NoopLoader;
use stc_ts_file_analyzer::env::EnvFactory;
use stc_ts_storage::Single;
use stc_ts_types::{module_id, Id, ModuleId, Type};
use std::path::Path;
use std::{error::Error, sync::Arc};
use swc_common::comments::NoopComments;
use swc_common::SourceFile;
use swc_common::{input::SourceFileInput, FileName, Mark, SourceMap, SyntaxContext};
use swc_ecma_ast::EsVersion;

pub fn new_check(file_path: &str) -> Result<(), Box<dyn Error>> {
  let cm: Arc<SourceMap> = Default::default();
  let fm = cm.load_file(Path::new(file_path))?;
  let comments = &NoopComments;
  let unresolved_marks = Mark::new();
  let top_level_mark = Mark::fresh(Mark::root());

  let module = stc_ts_testing::parse_rnode(&fm, comments, unresolved_marks, top_level_mark);
  let path = Arc::new(fm.name.clone());
  let generator = module_id::ModuleIdGenerator::default();
  let (module_id, top_level_mark) = generator.generate(&path);

  let mut storage = Single {
    parent: None,
    id: module_id,
    top_level_ctxt: SyntaxContext::empty().apply_mark(top_level_mark),
    path,
    is_dts: false,
    info: Default::default(),
  };

  let env = get_env();

  let box_storage = Box::new(&mut storage);

  let mut analyzer = Analyzer::root(env, cm, Default::default(), box_storage, &NoopLoader, None);

  module.visit_with(&mut analyzer);

  dbg!(module);

  Ok(())
}

fn source_file(path: &str) -> Result<Arc<SourceFile>, Box<dyn Error>> {
  let source_map: SourceMap = Default::default();
  let source_file = source_map.load_file(Path::new(path))?;

  Ok(source_file)
}

fn get_env() -> Env {
  let mut libs = vec![];
  let ls = &["es5"];
  for s in ls {
    libs.extend(Lib::load(s))
  }
  libs.sort();
  libs.dedup();

  Env::simple(
    Rule {
      strict_null_checks: true,
      strict_function_types: true,
      ..Default::default()
    },
    EsVersion::latest(),
    ModuleConfig::None,
    &libs,
  )
}
