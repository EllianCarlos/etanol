use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod server;
mod handlers;
mod syntax;
mod util;

use crate::server::KotlinLsp;

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let documents = Arc::new(RwLock::new(HashMap::new()));

    let (service, socket) = LspService::new(|client| KotlinLsp::new(client, Arc::clone(&documents)));
    Server::new(stdin, stdout, socket).serve(service).await
}
