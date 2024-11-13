use internal_baml_diagnostics::Span;

use crate::forms::identifier::Identifier;
use crate::forms::r#type::Type;
use crate::pos::WithPos;

#[derive(Clone, Debug)]
pub struct Class<T> {
    pub name: Identifier<T>,
    pub fields: Vec<Field<T>>,
    pub meta: T,
}

impl WithPos for Class<Span> {
    fn with_pos(self, pos: Span) -> Self {
        Class {
            meta: pos,
            ..self
        }
    }
}


#[derive(Clone, Debug)]
pub struct Field<T> {
    pub name: Identifier<T>,
    pub r#type: Type<T>,
    // attributes: Vec<Attribute<T>>,
    pub meta: T,
}

impl WithPos for Field<Span> {
    fn with_pos(self, pos: Span) -> Self {
        Field {
            meta: pos,
            ..self
        }
    }
}
