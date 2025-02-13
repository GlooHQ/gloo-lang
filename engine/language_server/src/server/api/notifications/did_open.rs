use std::collections::HashMap;
use std::path::{Path, PathBuf};
use baml_runtime::InternalRuntimeInterface;
use lsp_server::ErrorCode;
use lsp_types::notification::DidOpenTextDocument;
use lsp_types::{DiagnosticSeverity, DidOpenTextDocumentParams, PublishDiagnosticsParams, Url};

// use crate::baml_diagnostics::baml_to_lsp_diagnostics;
use crate::baml_project::watch::ChangeEvent;
use crate::baml_project::Project;
use crate::baml_text_size::TextSize;
use crate::server::api::traits::{NotificationHandler, SyncNotificationHandler};
use crate::server::client::{Notifier, Requester};
use crate::server::{Result, api::Error};
use crate::session::Session;
// use crate::system::{url_to_any_system_path, AnySystemPath};
use crate::{DocumentKey, TextDocument};

pub(crate) struct DidOpenTextDocumentHandler;

impl NotificationHandler for DidOpenTextDocumentHandler {
    type NotificationType = DidOpenTextDocument;
}

impl SyncNotificationHandler for DidOpenTextDocumentHandler {
    fn run(
        session: &mut Session,
        notifier: Notifier,
        _requester: &mut Requester,
        params: DidOpenTextDocumentParams,
    ) -> Result<()> {
        tracing::info!("DidOpenTextDocumentHandler");
        // let Ok(path) = url_to_any_system_path(&params.text_document.uri) else {
        //     return Ok(());
        // };

        eprintln!("ADDING DOCUMENT");
        let document = TextDocument::new(params.text_document.text, params.text_document.version);

        eprintln!("CALLING open_text_document");
        let url = params.text_document.uri;
        session.open_text_document(url.clone(), document);
        eprintln!("FINISHED CALLING open_text_document");
        // dbg!(&session.index.as_ref().map(|i| i.documents));

        let diagnostics = session_lsp_diagnostics(session);

        // TODO: Only send this when clients do not support pull diagnostics?
        notifier.notify::<lsp_types::notification::PublishDiagnostics>( PublishDiagnosticsParams {
            uri: url,
            version: Some(params.text_document.version),
            diagnostics,
        }).expect("TODO");

        eprintln!("PUSHED DIAGNOSTICS");

        // match path {
        //     AnySystemPath::System(path) => {
        //         let db = match session.project_db_for_path_mut(path.as_std_path()) {
        //             Some(db) => db,
        //             None => session.default_project_db_mut(),
        //         };
        //         db.apply_changes(vec![ChangeEvent::Opened(path)], None);
        //     }
        //     AnySystemPath::SystemVirtual(virtual_path) => {
        //         let db = session.default_project_db_mut();
        //         db.files().virtual_file(db, &virtual_path);
        //     }
        // }

        // TODO(dhruvmanila): Publish diagnostics if the client doesn't support pull diagnostics

        Ok(())
    }
}

// TODO: This assumes a single project. Fix.
// TODO: Handle errors.
fn session_lsp_diagnostics(session: &Session) -> Vec<lsp_types::Diagnostic> {
    let (root_path, proj) = session.projects_by_workspace_folder.iter().next().expect("Should be 1 project");
    dbg!(&proj.current_runtime.is_some());
    let fake_env = HashMap::new();
    let baml_diagnostics = match proj.baml_project.runtime(fake_env) {
        Ok(runtime) => {
            eprintln!("OK Diagnostics: {:?}", runtime.internal().diagnostics());
            runtime.internal().diagnostics().clone()
        },
        Err(err) => {
            eprintln!("Err Diagnostics: {:?}", err);
            // let mut diagnostics = internal_baml_diagnostics::Diagnostics::new(PathBuf::new());
            // diagnostics.push_error(err);
            err
        },
    };

    let errors = 
        baml_diagnostics.errors().iter().map(|error| lsp_types::Diagnostic::new(
            span_to_range(session, root_path, error.span()).expect("Need a range"),
            Some(DiagnosticSeverity::ERROR),
            None,
            None,
            error.message().to_string(),
            None,
            None
        ));
    let warnings =
        baml_diagnostics.warnings().iter().map(|warning| lsp_types::Diagnostic::new(
            span_to_range(session, root_path, warning.span()).expect("Need a range"),
            Some(DiagnosticSeverity::WARNING),
            None,
            None,
            warning.message().to_string(),
            None,
            None
        ));
    errors.chain(warnings).collect()
}

fn span_to_range(session: &Session, project_root: &Path, span: &internal_baml_diagnostics::Span) -> Option<lsp_types::Range> {
    dbg!(span.file.path().as_str());
    let absolute_path = span.file.path().clone();
    dbg!(&absolute_path);
    let path_buf = span.file.path_buf().clone();
    let url = Url::parse(absolute_path.as_str()).expect("Should parse");
    dbg!(session.index.as_ref());
    dbg!(session.index.as_ref().and_then(|i| i.documents.get(&url)));
    let doc = session.index.as_ref().and_then(|i| i.documents.get(&url)).expect("Should exist");
    let line_index = doc.as_text().unwrap().index();

    let start_loc = line_index.source_location(TextSize::new(span.start as u32), span.file.as_str());
    let end_loc = line_index.source_location(TextSize::new(span.end as u32), span.file.as_str());

    let (start_line, start_col) = (start_loc.row.get(), start_loc.column.get());
    let (end_line, end_col) = (end_loc.row.get(), end_loc.column.get());
    Some(lsp_types::Range{
        start: lsp_types::Position::new(start_line as u32, start_col as u32),
        end: lsp_types::Position::new(end_line as u32, end_col as u32),
    })
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_to_url() {
        let path_buf = PathBuf::from("file:///test.baml");
        let url = Url::from_file_path(&path_buf).unwrap();
        assert_eq!(url.as_str(), "file:///test.baml");
    }

    #[test]
    fn parse_path_with_file_prefix() {
        let url = Url::parse("file:///test.baml").unwrap();
        assert_eq!(url.as_str(), "file:///test.baml");
    }

}