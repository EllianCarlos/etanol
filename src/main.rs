use std::{env, fs};
use rnix::{Root, SyntaxKind, SyntaxNode};
use serde_json::{json, Value};

fn main() {
    let args = env::args().skip(1);
    if args.len() == 0 {
        eprintln!("Usage: etanol <config.nix>");
        return;
    }

    for file in args {
        match fs::read_to_string(&file) {
            Ok(content) => parse_nix_config(&content),
            Err(err) => eprintln!("Error reading {}: {}", file, err),
        }
    }
}

fn parse_nix_config(content: &str) {
    let parse = Root::parse(content);
    
    if !parse.errors().is_empty() {
        for error in parse.errors() {
            eprintln!("Parse error: {}", error);
        }
        return;
    }

    let extracted_data = extract_nix_config(&parse.syntax());
    println!("{}", serde_json::to_string_pretty(&extracted_data).unwrap());
}

fn extract_nix_config(root: &SyntaxNode) -> Value {
    let mut adapters = vec![];
    let mut tasks = vec![];

    for node in root.children() {
        if let Some((key, value_node)) = extract_key_value(node) {
            match key.as_str() {
                "adapters" => adapters = extract_list(&value_node),
                "tasks" => tasks = extract_tasks(&value_node),
                _ => {}
            }
        }
    }

    json!({ "adapters": adapters, "tasks": tasks })
}

fn extract_key_value(node: SyntaxNode) -> Option<(String, SyntaxNode)> {
    if node.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
        let key = node.first_child()?.text().to_string();
        let value_node = node.last_child()?;
        Some((key, value_node))
    } else {
        None
    }
}

fn extract_list(list_node: &SyntaxNode) -> Vec<String> {
    list_node
        .children()
        .filter_map(|child| {
            (child.kind() == SyntaxKind::NODE_LITERAL).then(|| child.text().to_string().trim_matches('"').to_string())
        })
        .collect()
}

fn extract_tasks(list_node: &SyntaxNode) -> Vec<Value> {
    list_node
        .children()
        .filter_map(|child| {
            (child.kind() == SyntaxKind::NODE_ATTR_SET).then(|| extract_task_attributes(&child))
        })
        .collect()
}

fn extract_task_attributes(task_node: &SyntaxNode) -> Value {
    let mut task = serde_json::Map::new();

    for attr in task_node.children().filter(|n| n.kind() == SyntaxKind::NODE_ATTRPATH_VALUE) {
        if let Some((key, value_node)) = extract_key_value(attr) {
            task.insert(key, Value::String(value_node.text().to_string()));
        }
    }

    Value::Object(task)
}

