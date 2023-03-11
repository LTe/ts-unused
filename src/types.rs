use std::convert::From;
use swc_common::{util::move_map::MoveMap, TypeEq};
use swc_ecma_ast::*;
use swc_ecma_visit::*;

#[derive(Default, Debug)]
pub struct TypescriptType {
    fields: Vec<Property>,
}

#[derive(Debug)]
struct Property {
    kind: String,
    name: String,
}

impl TypescriptType {
    pub fn new() -> TypescriptType {
        TypescriptType { fields: vec![] }
    }

    pub fn add(&mut self, property: Property) {
        self.fields.push(property)
    }
}

impl From<&TsTypeLit> for TypescriptType {
    fn from(item: &TsTypeLit) -> Self {
        let members = &item.members;
        let typescript_type = TypescriptType::new();

        let names: Vec<String> = members
            .iter()
            .map(|member| {
                let atom = match member {
                    TsTypeElement::TsPropertySignature(TsPropertySignature { key, .. }) => {
                        match *key.clone() {
                            Expr::Ident(Ident { sym, .. }) => sym,
                            _ => {
                                panic!()
                            }
                        }
                    }
                    _ => {
                        panic!()
                    }
                };

                atom.to_string()
            })
            .collect();

        dbg!(names);

        typescript_type
    }
}

pub struct Visitor;

impl Visit for Visitor {
    fn visit_ts_type(&mut self, ts_type: &TsType) {
        match ts_type {
            TsType::TsTypeLit(literal) => TypescriptType::from(literal),
            _ => TypescriptType::new(),
        };

        ()
    }
}
