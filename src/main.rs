use std::error::Error;
use ts_unused::checker::UnusedChecker;

fn main() -> Result<(), Box<dyn Error>> {
  let path = "./tests/data/test.ts";
  let unused_checker = UnusedChecker::check(path);
  let checker = unused_checker.create_stc_checker(path);

  dbg!(unused_checker);

  Ok(())
}
