use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;

pub async fn handle_completion(client: &Client, line: u32, character: u32) -> Result<Option<CompletionResponse>> {
    client
        .log_message(
            tower_lsp::lsp_types::MessageType::INFO,
            format!("Completion requested at line: {}, character: {}", line, character),
        )
        .await;

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
        CompletionItem {
            label: "fun".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("fun myFunction() { }".into()),
            ..CompletionItem::default()
        },
        CompletionItem {
            label: "if".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("if (condition) { }".into()),
            ..CompletionItem::default()
        },
        CompletionItem {
            label: "else".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("else { }".into()),
            ..CompletionItem::default()
        },
    ];

    // Return the completions as a response
    Ok(Some(CompletionResponse::Array(completions)))
}
