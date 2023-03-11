use swc_ecma_ast::*;
use swc_ecma_visit::*;

#[derive(Default, Debug)]
pub struct TypescriptType {
    name: String,
    fields: Vec<Property>,
}

#[derive(Debug)]
struct Property {
    name: String,
}

impl TryFrom<&TsTypeAliasDecl> for TypescriptType {
    type Error = &'static str;

    fn try_from(item: &TsTypeAliasDecl) -> Result<Self, Self::Error> {
        let name = item.id.sym.to_string();

        match *item.type_ann.clone() {
            TsType::TsTypeLit(literal) => {
                let members = &literal.members;

                let names: Vec<Property> = members
                    .iter()
                    .map(|member| {
                        let resolved_type = match member {
                            TsTypeElement::TsPropertySignature(TsPropertySignature {
                                key, ..
                            }) => match *key.clone() {
                                Expr::Ident(Ident { sym, .. }) => sym,
                                _ => {
                                    panic!()
                                }
                            },
                            _ => {
                                panic!()
                            }
                        };

                        Property {
                            name: resolved_type.to_string(),
                        }
                    })
                    .collect();

                Ok(TypescriptType {
                    name,
                    fields: names,
                })
            }
            _ => Err("Unsupported type"),
        }
    }
}

#[derive(Debug)]
pub struct Visitor {
    typescript_types: Vec<TypescriptType>,
}

impl Visitor {
    pub fn new() -> Self {
        Visitor {
            typescript_types: vec![],
        }
    }
}

impl Visit for Visitor {
    fn visit_ts_type_alias_decl(&mut self, type_alias: &TsTypeAliasDecl) {
        match TypescriptType::try_from(type_alias) {
            Ok(typescript_type) => self.typescript_types.push(typescript_type),
            Err(_) => { () }
        }
    }
}
