use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_kotlin() -> Language; }

pub fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language( unsafe { tree_sitter_kotlin() }).expect("Error loading Kotlin grammar.");
    parser
}
