use ts_unused::{
  checker::{NewUnusedChecker, UnusedChecker},
  visitor::{Property, TypescriptType},
};

#[test]
#[ignore]
fn test_single_file() {
  let path = "./tests/data/test.tsx";
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

#[test]
fn test_single_file_new() {
  let path = "./tests/data/test.tsx";
  let checker = NewUnusedChecker::check(path);
  let types = checker.typescript_types();
  let first_type = types.first().unwrap();

  assert_eq!(first_type.as_str(), "User");
}
