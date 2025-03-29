use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::handlers::initialization::handle_initialize;
use crate::handlers::initialization::handle_initialized;
use crate::handlers::completion::handle_completion;


#[derive(Debug)]
pub struct KotlinLsp {
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<String, String>>>,
}

impl KotlinLsp {
    pub fn new(client: Client, documents: Arc<RwLock<HashMap<String, String>>>) -> Self {
        KotlinLsp { client, documents }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for KotlinLsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, format!("Initialization params: {:?}", params))
            .await;

        handle_initialize(params).await
    }

    async fn initialized(&self, _: InitializedParams) {
        handle_initialized(&self.client).await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.handle_did_open(params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.handle_did_change(params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.handle_did_close(params).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        handle_completion(&self.client, params).await
    }
}
