use swc_ecma_visit::Visit;
use ts_unused::parser::SWCParser;
use ts_unused::visitor::{Property, TypescriptType, Visitor};

#[test]
fn test_single_file() {
    let path = "./tests/data/test.ts";
    let parser = SWCParser::new(path).unwrap();

    let mut visitor = Visitor::new();

    visitor.visit_module(&parser.module);

    assert_eq!(
        visitor.typescript_types,
        vec![TypescriptType {
            name: String::from("User"),
            fields: vec![
                Property {
                    name: String::from("username")
                },
                Property {
                    name: String::from("age")
                }
            ]
        }]
    );
}
