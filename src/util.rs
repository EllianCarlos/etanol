use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use tower_lsp::lsp_types::MessageType;
use crate::KotlinLsp;

/// Monotonic counter so every call gets its own file name, even when two
/// calls land in the same process-clock tick. Combined with the process ID,
/// this also keeps concurrent `etanol` instances from colliding.
static NEXT_TEMP_FILE_ID: AtomicU64 = AtomicU64::new(0);

pub async fn create_temp_file(lsp: &KotlinLsp, code: &str) -> Option<PathBuf> {
    let temp_dir = env::temp_dir();
    let id = NEXT_TEMP_FILE_ID.fetch_add(1, Ordering::Relaxed);
    let temp_file_path = temp_dir.join(format!("etanol_{}_{}.kts", std::process::id(), id));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::test_lsp;

    // Regression test for a real bug: create_temp_file used to always write to
    // the same fixed path (temp_dir.join("temp_kotlin_script.kts")), so two
    // documents open at once would silently overwrite each other's syntax-check
    // input. This is deterministic — no need for real concurrency to prove it,
    // since the old code returned the identical path on every single call.
    #[tokio::test]
    async fn each_call_gets_a_distinct_temp_file() {
        let (service, _documents) = test_lsp();
        let lsp = service.inner();

        let path_a = create_temp_file(lsp, "// document A")
            .await
            .expect("create_temp_file should succeed for document A");
        let path_b = create_temp_file(lsp, "// document B")
            .await
            .expect("create_temp_file should succeed for document B");

        assert_ne!(
            path_a, path_b,
            "each document must get its own temp file, or concurrent edits corrupt each other's content"
        );

        assert_eq!(std::fs::read_to_string(&path_a).unwrap(), "// document A");
        assert_eq!(std::fs::read_to_string(&path_b).unwrap(), "// document B");

        let _ = std::fs::remove_file(&path_a);
        let _ = std::fs::remove_file(&path_b);
    }
}
