use ts_unused::{
  checker::UnusedChecker,
  visitor::{Property, TypescriptType},
};

#[test]
fn test_single_file() {
  let path = "./tests/data/test.ts";
  let checker = UnusedChecker::check(path);

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
