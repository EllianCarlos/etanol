use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use tower_lsp::lsp_types::MessageType;
use crate::KotlinLsp;

pub async fn create_temp_file(lsp: &KotlinLsp, code: &str) -> Option<PathBuf> {
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("temp_kotlin_script.kts");

    match File::create(&temp_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                lsp.client
                    .log_message(MessageType::ERROR, format!("Failed to write to temp file: {}", e))
                    .await;
                return None;
            }
            Some(temp_file_path)
        }
        Err(e) => {
            lsp.client
                .log_message(MessageType::ERROR, format!("Failed to create temp file: {}", e))
                .await;
            None
        }
    }
}
