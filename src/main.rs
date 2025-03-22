use tower_lsp::jsonrpc::Result;
use tower_lsp::{lsp_types::*, Client, LanguageServer, LspService, Server};
use tokio::io::{stdin, stdout};

#[derive(Debug)]
struct KotlinLsp {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for KotlinLsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, format!("Initialization params: {:?}", params))
            .await;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions::default()),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Kotlin LSP initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = vec![
            CompletionItem {
                label: "println".into(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("println(\"Hello, Kotlin!\")".into()),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "val".into(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("val name = \"Kotlin\"".into()),
                ..CompletionItem::default()
            },
        ];

        Ok(Some(CompletionResponse::Array(completions)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let (service, socket) = LspService::new(|client| KotlinLsp { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
