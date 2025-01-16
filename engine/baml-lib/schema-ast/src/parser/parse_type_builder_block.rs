use super::{
    helpers::{parsing_catch_all, Pair},
    parse_attribute::parse_attribute,
    parse_comments::*,
    parse_field::parse_value_expr,
    parse_identifier::parse_identifier,
    parse_named_args_list::{parse_function_arg, parse_named_argument_list},
    Rule,
};

use crate::ast::*;
use internal_baml_diagnostics::{DatamodelError, Diagnostics};

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

pub struct TypeBuilderBlock {
    pub entries: Vec<TypeBuilderEntry>,
}

pub(crate) fn parse_type_builder_block(
    pair: Pair<'_>,
    doc_comment: Option<Pair<'_>>,
    diagnostics: &mut Diagnostics,
) -> Result<ValueExprBlock, DatamodelError> {
}
