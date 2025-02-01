//! ## The Ruff Language Server

pub use edit::{DocumentKey, PositionEncoding, TextDocument};
use lsp_types::CodeActionKind;
pub use server::{Server, Workspace, Workspaces};
pub use session::{ClientSettings, DocumentQuery, DocumentSnapshot, Session};

#[macro_use]
mod message;

mod baml_diagnostics;
mod baml_linter;
mod baml_source_file;
mod baml_text_size;
mod edit;
mod fix;
mod format;
mod lint;
mod logging;
mod resolve;
mod server;
mod session;

pub(crate) const SERVER_NAME: &str = "baml";
pub(crate) const DIAGNOSTIC_NAME: &str = "BAML";

pub(crate) const SOURCE_FIX_ALL_BAML: CodeActionKind = CodeActionKind::new("source.fixAll.baml");
pub(crate) const SOURCE_ORGANIZE_IMPORTS_BAML: CodeActionKind =
    CodeActionKind::new("source.organizeImports.baml");
pub(crate) const NOTEBOOK_SOURCE_FIX_ALL_BAML: CodeActionKind =
    CodeActionKind::new("notebook.source.fixAll.baml");
pub(crate) const NOTEBOOK_SOURCE_ORGANIZE_IMPORTS_BAML: CodeActionKind =
    CodeActionKind::new("notebook.source.organizeImports.baml");

/// A common result type used in most cases where a
/// result type is needed.
pub(crate) type Result<T> = anyhow::Result<T>;

pub(crate) fn version() -> &'static str {
    // ruff_linter::VERSION
    "0.0.1"
}
