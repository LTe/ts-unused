use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct TypescriptType {
  name: String,
  fields: Vec<Property>,
}

impl TypescriptType {
  pub fn new(name: String, fields: Vec<Property>) -> Self {
    TypescriptType { name, fields }
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Property {
  name: String,
}

impl Property {
  pub fn new(name: String) -> Self {
    Property { name }
  }
}

impl TryFrom<&TsTypeAliasDecl> for TypescriptType {
  type Error = &'static str;

  fn try_from(item: &TsTypeAliasDecl) -> Result<Self, Self::Error> {
    let name = item.id.sym.to_string();
    let type_annotation = item.type_ann.clone();

    let resolve_properties = |member: &TsTypeElement| match member {
      TsTypeElement::TsPropertySignature(TsPropertySignature { key, .. }) => match *key.clone() {
        Expr::Ident(Ident { sym, .. }) => Some(Property {
          name: sym.to_string(),
        }),
        _ => None,
      },
      _ => None,
    };

    match *type_annotation {
      TsType::TsTypeLit(literal) => {
        let members = &literal.members;
        let names = members.iter().map(resolve_properties).flatten().collect();

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

  pub fn add(&mut self, typescript_type: TypescriptType) -> &mut Self {
    self.typescript_types.push(typescript_type);

    self
  }

  pub fn typescript_types(self) -> Vec<TypescriptType> {
    self.typescript_types
  }
}

impl Visit for Visitor {
  fn visit_ts_type_alias_decl(&mut self, type_alias: &TsTypeAliasDecl) {
    match TypescriptType::try_from(type_alias) {
      Ok(typescript_type) => Some(self.add(typescript_type)),
      Err(_) => None,
    };
  }
}
