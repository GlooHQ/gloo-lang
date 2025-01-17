use super::{Assignment, TypeExpressionBlock};

#[derive(Debug, Clone)]
/// Blocks allowed in `type_builder` blocks.
pub enum TypeBuilderEntry {
    /// An enum declaration.
    Enum(TypeExpressionBlock),
    /// A class declaration.
    Class(TypeExpressionBlock),
    /// Type alias expression.
    TypeAlias(Assignment),
    /// Dynamic block.
    Dynamic(TypeExpressionBlock),
}

#[derive(Debug, Clone)]
pub struct TypeBuilderBlock {
    pub entries: Vec<TypeBuilderEntry>,
}
