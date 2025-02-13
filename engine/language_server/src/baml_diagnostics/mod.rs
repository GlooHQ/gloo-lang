pub use diagnostic::{Diagnostic, DiagnosticKind};
pub use edit::Edit;
pub use fix::{Applicability, Fix, IsolationLevel};
use internal_baml_diagnostics::{self, DatamodelError, DatamodelWarning};
pub use source_map::{SourceMap, SourceMarker};
pub use violation::{AlwaysFixableViolation, FixAvailability, Violation, ViolationMetadata};

use crate::baml_text_size::{TextRange, TextSize};

pub mod diagnostic;
mod edit;
mod fix;
mod source_map;
mod violation;

// pub fn baml_to_lsp_diagnostics(
//     diagnostics: internal_baml_diagnostics::Diagnostics,
// ) -> Vec<Diagnostic> {
//     diagnostics
//         .errors()
//         .iter()
//         .map(|error: &DatamodelError| {
//             Diagnostic::new(
//                 DiagnosticKind {
//                     name: "error".to_string(),
//                     body: error.message().to_string(),
//                     suggestion: None,
//                 },
//                 error.span().into(),
//             )
//         })
//         .chain(diagnostics.warnings().iter().map(|warning| {
//             Diagnostic::new(
//                 DiagnosticKind {
//                     name: "warning".to_string(),
//                     body: warning.message().to_string(),
//                     suggestion: None
//                 },
//                warning.span().into()
//             )
//         }))
//         .collect()
// }
// 
// impl From<&internal_baml_diagnostics::Span> for TextRange {
//     fn from(
//         span: &internal_baml_diagnostics::Span,
//     ) -> TextRange {
//         TextRange::new( TextSize::new(span.start as u32), TextSize::new(span.end as u32))
//     }
// }
// 