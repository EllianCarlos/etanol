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
