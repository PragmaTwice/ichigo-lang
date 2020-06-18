use crate::syntax::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: Ident,
    pub optional_type: Option<Type>,
    pub instance_from: Option<Ident>,
}

impl Symbol {
    pub fn new(id: &Ident) -> Self {
        Self {
            id: id.clone(),
            optional_type: None,
            instance_from: None,
        }
    }

    pub fn new_with_type(id: &Ident, type_: &Type) -> Self {
        Self {
            id: id.clone(),
            optional_type: Some(type_.clone()),
            instance_from: None,
        }
    }

    pub fn from_instance(instance: &Instance, from_type: &Ident) -> Self {
        let Instance { id, type_ } = instance;

        Self {
            id: id.clone(),
            optional_type: Some(type_.as_ref().clone()),
            instance_from: Some(from_type.clone()),
        }
    }
}
