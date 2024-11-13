use internal_baml_diagnostics::Span;

use crate::forms::identifier::Identifier;
use crate::pos::WithPos;

// TODO: Fill this out.
#[derive(Clone, Debug)]
pub enum Expression<T> {
    Variable{ identifier: Identifier<T> },
    QuotedString { inner: String },
    RawString { inner: String },
}
