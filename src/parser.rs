use rnix_parser::parse;
use rnix_parser::SyntaxNode;

pub fn parse_nix(nix_code: &str) -> Vec<String> {
    let ast = parse(nix_code);
    let mut tasks = vec![];

    if let Some(root) = ast.root().and_then(|n| Some(n.into_syntax())) {
        extract_tasks(&root, &mut tasks);
    }

    tasks
}

fn extract_tasks(ast: &SyntaxNode, tasks: &mut Vec<String>) {
    for node in ast.children() {
        if let Some(identifier) = node.text().strip_prefix("tasks.") {
            tasks.push(identifier.to_string());
        }
    }
}

