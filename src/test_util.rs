//! Shared test-only helpers: an LSP test harness and Kotlin fixture loaders.
//!
//! Only compiled under `cargo test` (see the `#[cfg(test)]` gate on the `mod
//! test_util;` declaration in `main.rs`).

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use futures::StreamExt;
use tokio::sync::RwLock;
use tower_lsp::LspService;

use crate::KotlinLsp;

/// Builds a real `KotlinLsp` wired to a fresh, empty document store.
///
/// `tower-lsp` only hands out a usable `Client` through `LspService::new`, so
/// this goes through the real service construction rather than a hand-rolled
/// mock. The service's outgoing channel is drained in the background so that
/// `log_message`/`publish_diagnostics` calls inside handlers never block on
/// the channel's backpressure (it has a capacity of 1).
///
/// Call `service.inner()` to get a `&KotlinLsp` and invoke handler methods
/// directly. Returns the shared document store too, so tests can assert on
/// it without going through the LSP protocol layer.
pub fn test_lsp() -> (LspService<KotlinLsp>, Arc<RwLock<HashMap<String, String>>>) {
    let documents = Arc::new(RwLock::new(HashMap::new()));
    let store = Arc::clone(&documents);
    let (service, socket) = LspService::new(move |client| KotlinLsp::new(client, Arc::clone(&store)));

    tokio::spawn(socket.for_each(|_| async {}));

    (service, documents)
}

/// Loads every `.kt` file under `tests/fixtures/gasoline/` as a `(file name, source)` pair.
///
/// These are real, hand-written Kotlin files vendored from the `gasoline` test
/// framework repository, used as a "does real-world code break the LSP" smoke corpus.
pub fn load_gasoline_fixtures() -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/gasoline");
    read_files_with_extension(&dir, "kt")
}

/// Loads every test case out of the vendored `tree-sitter-kotlin` grammar's own
/// corpus files as `("<file>::<test name>", source)` pairs.
///
/// This is a pragmatic parser of the standard tree-sitter corpus format
/// (`====\n<name>\n====\n<source>\n----\n<expected sexp>`) that extracts just the
/// Kotlin source of each case. It does not attempt exact-sexp fidelity with the
/// `tree-sitter test` CLI — callers should only assert parse-without-error, not
/// an exact tree shape.
pub fn load_corpus_fixtures() -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor/tree-sitter-kotlin/test/corpus");
    let mut fixtures = Vec::new();

    for (file_name, content) in read_files_with_extension(&dir, "txt") {
        for (case_name, source) in parse_corpus_file(&content) {
            fixtures.push((format!("{file_name}::{case_name}"), source));
        }
    }

    fixtures
}

fn read_files_with_extension(dir: &Path, extension: &str) -> Vec<(String, String)> {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read fixture dir {dir:?}: {e}"));

    let mut files = Vec::new();
    for entry in entries {
        let path = entry.expect("failed to read fixture dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some(extension) {
            continue;
        }

        let name = path
            .file_stem()
            .expect("fixture file has no name")
            .to_string_lossy()
            .to_string();
        let content =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));

        files.push((name, content));
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

/// Splits one tree-sitter corpus file into `(test name, Kotlin source)` cases.
fn parse_corpus_file(content: &str) -> Vec<(String, String)> {
    fn is_rule_line(line: &str, marker: char) -> bool {
        line.len() >= 3 && line.chars().all(|c| c == marker)
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut cases = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if !is_rule_line(lines[i], '=') {
            i += 1;
            continue;
        }
        i += 1; // opening "====" rule

        let mut name_lines = Vec::new();
        while i < lines.len() && !is_rule_line(lines[i], '=') {
            name_lines.push(lines[i]);
            i += 1;
        }
        i += 1; // closing "====" rule
        let name = name_lines.join(" ").trim().to_string();

        let mut source_lines = Vec::new();
        while i < lines.len() && !is_rule_line(lines[i], '-') {
            source_lines.push(lines[i]);
            i += 1;
        }
        i += 1; // "----" rule separating source from expected sexp

        // Skip the expected S-expression until the next case header or EOF.
        while i < lines.len() && !is_rule_line(lines[i], '=') {
            i += 1;
        }

        let source = source_lines.join("\n").trim().to_string();
        if !source.is_empty() {
            cases.push((name, source));
        }
    }

    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_corpus_file_extracts_named_cases() {
        let content = "\
==================
Top-level function
==================

fun main() {}

---

(source_file
  (function_declaration))

==================
Empty class
==================

class Foo

---

(source_file
  (class_declaration))
";

        let cases = parse_corpus_file(content);

        assert_eq!(
            cases,
            vec![
                ("Top-level function".to_string(), "fun main() {}".to_string()),
                ("Empty class".to_string(), "class Foo".to_string()),
            ]
        );
    }

    #[test]
    fn load_corpus_fixtures_finds_vendored_cases() {
        let fixtures = load_corpus_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected at least one case from vendor/tree-sitter-kotlin/test/corpus"
        );
    }

    #[test]
    fn load_gasoline_fixtures_finds_vendored_files() {
        let fixtures = load_gasoline_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected at least one .kt file under tests/fixtures/gasoline"
        );
    }

    #[tokio::test]
    async fn test_lsp_documents_store_starts_empty() {
        let (_service, documents) = test_lsp();
        assert!(documents.read().await.is_empty());
    }
}
