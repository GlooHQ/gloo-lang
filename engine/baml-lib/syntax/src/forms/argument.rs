use crate::forms::Identifier;
use crate::forms::Type;

pub struct Argument<T> {
    pub name: Identifier<T>,
    pub r#type: Type<T>,
}
