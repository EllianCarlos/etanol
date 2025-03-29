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
