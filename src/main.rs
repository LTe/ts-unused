use stc_ts_type_checker::loader::LoadModule;
use stc_ts_types::Type;
use std::{error::Error, path::PathBuf, sync::Arc};
use swc_common::FileName;
use swc_ecma_ast::Lit;
use ts_unused::checker::UnusedChecker;

fn main() -> Result<(), Box<dyn Error>> {
  swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
    let path = "./tests/data/test.ts";
    let unused_checker = UnusedChecker::check(path);

    dbg!(unused_checker);
  });

  Ok(())
}
