use lsp_types::notification::DidOpenTextDocument;
use lsp_types::{DidOpenTextDocumentParams, PublishDiagnosticsParams, Url};

use crate::baml_project::watch::ChangeEvent;
use crate::server::api::traits::{NotificationHandler, SyncNotificationHandler};
use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
// use crate::system::{url_to_any_system_path, AnySystemPath};
use crate::TextDocument;

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

        let document = TextDocument::new(params.text_document.text, params.text_document.version);
        session.open_text_document(params.text_document.uri.clone(), document);

        // TODO: Only send this when clients do not support pull diagnostics?
        notifier.notify::<lsp_types::notification::PublishDiagnostics>( PublishDiagnosticsParams {
            uri: params.text_document.uri,
            version: Some(params.text_document.version),
            diagnostics: vec![],
        }).expect("TODO");

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
