use ts_unused::checker::Checker;
use ts_unused::visitor::{Property, TypescriptType};

#[test]
fn test_single_file() {
  let path = "./tests/data/test.ts";
  let checker = Checker::check(path);

  assert_eq!(
    checker.typescript_types(),
    vec![TypescriptType::new(
      String::from("User"),
      vec![
        Property::new(String::from("username")),
        Property::new(String::from("age"))
      ]
    )]
  );
}
