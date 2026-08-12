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

        check_syntax(self, &text, &uri).await;
    }

    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(change) = params.content_changes.first() {
            let text = change.text.clone();

            self.client
                .log_message(MessageType::INFO, format!("New text: {}", text))
                .await;

            self.documents.write().await.insert(uri.clone(), text.clone());

            check_syntax(self, &text, &uri).await;
        }
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        self.documents.write().await.remove(&uri);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::test_lsp;

    fn uri(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    fn open_params(u: &str, text: &str) -> DidOpenTextDocumentParams {
        DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri(u),
                language_id: "kotlin".into(),
                version: 1,
                text: text.into(),
            },
        }
    }

    fn full_text_change(text: &str) -> TextDocumentContentChangeEvent {
        TextDocumentContentChangeEvent { range: None, range_length: None, text: text.into() }
    }

    fn change_params(u: &str, changes: Vec<TextDocumentContentChangeEvent>) -> DidChangeTextDocumentParams {
        DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri: uri(u), version: 2 },
            content_changes: changes,
        }
    }

    fn close_params(u: &str) -> DidCloseTextDocumentParams {
        DidCloseTextDocumentParams { text_document: TextDocumentIdentifier { uri: uri(u) } }
    }

    #[tokio::test]
    async fn did_open_stores_document_text() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_open(open_params("file:///a.kt", "fun main() {}")).await;

        let docs = documents.read().await;
        assert_eq!(docs.get("file:///a.kt").map(String::as_str), Some("fun main() {}"));
    }

    #[tokio::test]
    async fn did_change_with_full_text_updates_document() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_open(open_params("file:///a.kt", "fun main() {}")).await;
        lsp.handle_did_change(change_params(
            "file:///a.kt",
            vec![full_text_change("fun main() { println(1) }")],
        ))
        .await;

        let docs = documents.read().await;
        assert_eq!(
            docs.get("file:///a.kt").map(String::as_str),
            Some("fun main() { println(1) }")
        );
    }

    // Characterizes current behavior: an empty content_changes list is
    // silently ignored, leaving the stored document untouched. A conforming
    // FULL-sync client should never send this, but a misbehaving one could —
    // this pins down what happens if it does.
    #[tokio::test]
    async fn did_change_with_no_content_changes_is_a_no_op() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_open(open_params("file:///a.kt", "fun main() {}")).await;
        lsp.handle_did_change(change_params("file:///a.kt", vec![])).await;

        let docs = documents.read().await;
        assert_eq!(docs.get("file:///a.kt").map(String::as_str), Some("fun main() {}"));
    }

    // Characterizes current behavior: there is no guard against out-of-order
    // notifications, so a didChange for a document that was never opened
    // just creates it.
    #[tokio::test]
    async fn did_change_for_unopened_document_inserts_it() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_change(change_params(
            "file:///never-opened.kt",
            vec![full_text_change("val x = 1")],
        ))
        .await;

        let docs = documents.read().await;
        assert_eq!(
            docs.get("file:///never-opened.kt").map(String::as_str),
            Some("val x = 1")
        );
    }

    #[tokio::test]
    async fn did_close_removes_document() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_open(open_params("file:///a.kt", "fun main() {}")).await;
        lsp.handle_did_close(close_params("file:///a.kt")).await;

        assert!(!documents.read().await.contains_key("file:///a.kt"));
    }

    #[tokio::test]
    async fn did_close_for_unknown_document_does_not_panic() {
        let (service, documents) = test_lsp();
        let lsp = service.inner();

        lsp.handle_did_close(close_params("file:///never-opened.kt")).await;

        assert!(documents.read().await.is_empty());
    }
}
