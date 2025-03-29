use crate::KotlinLsp;
use tower_lsp::lsp_types::*;

use crate::syntax::check_syntax;

impl KotlinLsp {
    pub async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text.clone();

        self.documents.write().await.insert(uri.clone(), text.clone());

        self.client
            .log_message(MessageType::INFO, "Did open document")
            .await;

        check_syntax(&self, &text, &uri).await;
    }

    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(change) = params.content_changes.first() {
            let text = change.text.clone();

            self.client
                .log_message(MessageType::INFO, format!("New text: {}", text))
                .await;

            self.documents.write().await.insert(uri.clone(), text.clone());

            check_syntax(&self, &text, &uri).await;
        }
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        self.documents.write().await.remove(&uri);
    }
}
