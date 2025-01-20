use internal_baml_diagnostics::Span;

use super::{Assignment, TypeExpressionBlock};

/// Blocks allowed in `type_builder` blocks.
#[derive(Debug, Clone)]
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
    pub span: Span,
}
