use tower_lsp::Client;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub async fn handle_initialize(_: InitializeParams) -> Result<InitializeResult> {
    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            completion_provider: Some(CompletionOptions::default()),
            ..ServerCapabilities::default()
        },
        ..InitializeResult::default()
    })
}

pub async fn handle_initialized(client: &Client) {
    client
        .log_message(MessageType::INFO, "Kotlin LSP initialized!")
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    // Locks in the current hardcoded response: handle_initialize ignores its
    // InitializeParams input entirely and always advertises the same
    // capabilities, regardless of what the client asked for or can do.
    #[tokio::test]
    async fn advertises_full_sync_and_basic_completion() {
        let result = handle_initialize(InitializeParams::default()).await.unwrap();

        assert_eq!(
            result.capabilities.text_document_sync,
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL))
        );
        assert!(result.capabilities.completion_provider.is_some());
    }

    #[tokio::test]
    async fn handle_initialized_does_not_panic() {
        let (service, _documents) = crate::test_util::test_lsp();
        handle_initialized(&service.inner().client).await;
    }
}
