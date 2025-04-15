extern crate cc;

fn main() {
    cc::Build::new()
        .include("vendor/tree-sitter-kotlin/src")
        .file("vendor/tree-sitter-kotlin/src/parser.c")
        .file("vendor/tree-sitter-kotlin/src/scanner.c")
        .compile("tree-sitter-kotlin");
}
