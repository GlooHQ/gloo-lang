use internal_baml_diagnostics::{SourceFile, Span};

use std::sync::Arc;

pub fn mk_pos(source_file: &SourceFile, left: usize, right: usize) -> Span {
    Span::new(
        source_file.clone(),
        left,
        right,
    )
}

pub fn empty_pos(source_file: &SourceFile) -> Span {
    mk_pos(source_file, 0,0)
}

pub trait WithPos {
    fn with_pos(self, pos: Span) -> Self;
}
