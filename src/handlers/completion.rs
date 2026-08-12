use tower_lsp::jsonrpc::Result;
use tower_lsp::jsonrpc::Error as JsonRpcError;
use tower_lsp::jsonrpc::ErrorCode::ServerError as Code;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tokio::fs;
use tokio::io::AsyncReadExt;
use std::borrow::Cow;

pub async fn handle_completion(client: &Client, params: CompletionParams) -> Result<Option<CompletionResponse>> {
    client
        .log_message(
            MessageType::INFO,
            format!("Completion requested at line: {}, character: {}", params.text_document_position.position.line, params.text_document_position.position.character),
        )
        .await;

    let document_content = match fetch_document_content(&params.text_document_position.text_document.uri).await {
        Ok(content) => content,
        Err(e) => {
            client.log_message(MessageType::ERROR, format!("Failed to read document content: {}", e)).await;
            return Ok(None);
        }
    };

    client
        .log_message(
            tower_lsp::lsp_types::MessageType::INFO,
            format!("Code: {}", document_content)
        )
        .await;

    let context = get_code_context(&document_content, params.text_document_position.position.line);
    let mut completions = Vec::new();

    match context {
        Context::Keyword => {
            completions.push(create_keyword_completion("val", "val name = \"Kotlin\""));
            completions.push(create_keyword_completion("fun", "fun myFunction() { }"));
            completions.push(create_keyword_completion("if", "if (condition) { }"));
            completions.push(create_keyword_completion("else", "else { }"));
        },
        Context::Function => {
            completions.push(create_function_completion("println", "println(\"Hello, Kotlin!\")"));
            completions.push(create_function_completion("print", "print(\"Hello\")"));
        },
        Context::Variable => {
            completions.push(create_variable_completion("name", "val name = \"Kotlin\""));
        }
    }

    Ok(Some(CompletionResponse::Array(completions)))
}


fn get_code_context(code: &str, line: u32) -> Context {
    let context_regex = regex::Regex::new(r"\bval\b|\bfun\b|\bif\b|\belse\b|\bprintln\b|\bprint\b|").unwrap();

    if context_regex.is_match(&code[line as usize..]) {
        if code.contains("fun") {
            return Context::Function;
        } else if code.contains("val") {
            return Context::Variable;
        }
    }

    Context::Keyword
}

async fn fetch_document_content(uri: &Url) -> Result<String> {
    let path = uri.to_file_path().map_err(|_| { JsonRpcError{ code: Code(1), message: Cow::Borrowed("Invalid URI, cannot convert to file path"), data: None } })?;
    let mut file = match fs::File::open(path).await {
        Ok(file) => file,
        Err(e) => return Err(JsonRpcError{ code: Code(1), message: Cow::Owned(format!("Failed to open file: {}", e)), data: None })
    };

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content).await {
        return Err(JsonRpcError{ code: Code(1), message: Cow::Owned(format!("Failed to read file content: {}", e)), data: None })
    }

    Ok(content)
}

enum Context {
    Keyword,
    Function,
    Variable,
}

fn create_keyword_completion(label: &str, insert_text: &str) -> CompletionItem {
    CompletionItem {
        label: label.into(),
        kind: Some(CompletionItemKind::KEYWORD),
        insert_text: Some(insert_text.into()),
        documentation: Some(Documentation::String(format!("Insert a {} keyword", label))),
        ..CompletionItem::default()
    }
}

fn create_function_completion(label: &str, insert_text: &str) -> CompletionItem {
    CompletionItem {
        label: label.into(),
        kind: Some(CompletionItemKind::FUNCTION),
        insert_text: Some(insert_text.into()),
        documentation: Some(Documentation::String(format!("Insert a function call for {}", label))),
        ..CompletionItem::default()
    }
}

fn create_variable_completion(label: &str, insert_text: &str) -> CompletionItem {
    CompletionItem {
        label: label.into(),
        kind: Some(CompletionItemKind::VARIABLE),
        insert_text: Some(insert_text.into()),
        documentation: Some(Documentation::String(format!("Insert a variable named {}", label))),
        ..CompletionItem::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::test_lsp;

    #[test]
    fn detects_function_keyword_context() {
        assert!(matches!(get_code_context("fun main() {}", 0), Context::Function));
    }

    #[test]
    fn detects_variable_keyword_context() {
        assert!(matches!(get_code_context("val x = 1", 0), Context::Variable));
    }

    #[test]
    fn falls_back_to_keyword_context_for_code_without_val_or_fun() {
        assert!(matches!(get_code_context("println(1)", 0), Context::Keyword));
    }

    #[test]
    fn empty_code_falls_back_to_keyword_context() {
        assert!(matches!(get_code_context("", 0), Context::Keyword));
    }

    // KNOWN BUG: every caller (see handle_completion below) passes an LSP
    // *line number* (params.text_document_position.position.line) as the
    // `line` argument, but get_code_context slices the source with it as a
    // *byte offset* instead (`&code[line as usize..]`). Any completion
    // request where the document is shorter than the cursor's line number
    // panics the request. This test pins the crash down explicitly so a fix
    // shows up as a test failure to update, not a silent behavior change.
    #[test]
    #[should_panic]
    fn line_number_used_as_byte_offset_panics_on_short_documents() {
        let code = "val x = 1"; // 9 bytes
        let _ = get_code_context(code, 100); // "line 100" treated as byte offset 100
    }

    fn write_temp_kotlin_file(contents: &str) -> (Url, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!(
            "etanol_completion_test_{}_{}.kt",
            std::process::id(),
            contents.len()
        ));
        std::fs::write(&path, contents).expect("failed to write temp fixture file");
        (Url::from_file_path(&path).unwrap(), path)
    }

    #[tokio::test]
    async fn handle_completion_returns_function_completions_for_fun_context() {
        let (service, _documents) = test_lsp();
        let (uri, path) = write_temp_kotlin_file("fun main() {}");

        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line: 0, character: 0 },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        };

        let response = handle_completion(&service.inner().client, params).await.unwrap();
        let _ = std::fs::remove_file(&path);

        match response {
            Some(CompletionResponse::Array(items)) => {
                assert!(!items.is_empty());
                assert!(items.iter().any(|i| i.label == "println"));
            }
            other => panic!("expected a non-empty completion array, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_completion_returns_none_for_missing_document() {
        let (service, _documents) = test_lsp();

        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("file:///does/not/exist.kt").unwrap(),
                },
                position: Position { line: 0, character: 0 },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        };

        let response = handle_completion(&service.inner().client, params).await.unwrap();
        assert!(response.is_none());
    }
}
