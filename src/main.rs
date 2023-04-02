use stc_ts_type_checker::loader::LoadModule;
use stc_ts_types::Type;
use std::{error::Error, path::PathBuf, sync::Arc};
use swc_common::FileName;
use swc_ecma_ast::Lit;
use ts_unused::{
  checker::{NewUnusedChecker, UnusedChecker},
  new_checker,
};

fn main() -> Result<(), Box<dyn Error>> {
  // swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
  //   let path = "./tests/data/test.tsx";
  //   let new_unused_checker = NewUnusedChecker::check(path);
  // });

  swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
    let result = new_checker::new_check("./tests/data/test.tsx");
    dbg!(result);
  });

  Ok(())
}
