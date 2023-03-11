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

        let names: Vec<Property> = members
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

                Property {
                    name: atom.to_string(),
                    kind: String::from("string"),
                }
            })
            .collect();

        TypescriptType { fields: names }
    }
}

#[derive(Debug)]
pub struct Visitor {
    typescript_types: Vec<TypescriptType>,
}

impl Visitor {
    pub fn new() -> Self {
        Visitor { typescript_types: vec![] }
    }
}

impl Visit for Visitor {
    fn visit_ts_type(&mut self, ts_type: &TsType) {
        let resolved_type = match ts_type {
            TsType::TsTypeLit(literal) => TypescriptType::from(literal),
            _ => TypescriptType::new(),
        };

        self.typescript_types.push(resolved_type)
    }
}
