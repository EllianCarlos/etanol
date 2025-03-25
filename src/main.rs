use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::{lsp_types::*, Client, LanguageServer, LspService, Server};
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
struct KotlinLsp {
    client: Client,
    documents: Arc<RwLock<HashMap<String, String>>>,
}

impl KotlinLsp {
    async fn check_syntax(&self, code: &str) {
        let temp_file = self.create_temp_file(code).await;

        if let Some(temp_path) = temp_file {
            let child = match Command::new("kotlinc")
                .arg("-script")
                .arg(temp_path.to_str().unwrap())
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        self.client
                            .log_message(MessageType::ERROR, format!("Failed to start kotlinc: {}", e))
                            .await;
                        return;
                    }
                };

            match child.wait_with_output() {
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stderr.is_empty() {
                        self.client
                            .log_message(MessageType::INFO, "No syntax errors")
                            .await;
                    } else {
                        self.client
                            .log_message(MessageType::WARNING, format!("Syntax Errors: \n{}", stderr))
                            .await;
                    }
                }
                Err(e) => {
                    self.client
                        .log_message(MessageType::ERROR, format!("Failed to read kotlinc output: {}", e))
                        .await;
                }
            }
        }

    }

    async fn create_temp_file(&self, code: &str) -> Option<PathBuf> {
        let temp_dir = env::temp_dir();
        let temp_file_path = temp_dir.join("temp_kotlin_script.kts");

        match File::create(&temp_file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(code.as_bytes()) {
                    self.client
                        .log_message(MessageType::ERROR, format!("Failed to write to temp file: {}", e))
                        .await;
                    return None;
                }
                Some(temp_file_path)
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Failed to create temp file: {}", e))
                    .await;
                None
            }
        }
    }
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text.clone();

        self.documents.write().await.insert(uri.clone(), text.clone());

        self.client
            .log_message(MessageType::INFO, "Did open document")
            .await;

        self.check_syntax(&text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(change) = params.content_changes.first() {
            let text = change.text.clone();

            self.client
                .log_message(MessageType::INFO, format!("New text: {}", text))
                .await;

            self.documents.write().await.insert(uri.clone(), text.clone());

            self.check_syntax(&text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        self.documents.write().await.remove(&uri);
    }
}

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let documents = Arc::new(RwLock::new(HashMap::new()));
    let (service, socket) = LspService::new(|client| KotlinLsp { client, documents: Arc::clone(&documents) });
    Server::new(stdin, stdout, socket).serve(service).await;
}
