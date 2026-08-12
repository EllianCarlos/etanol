# Architectural (?) Decision Record

- [2025-03-24] Using `kotlinc` process instead of a in memory parser to speed up development.
- [2025-04-14] Using interop with tree-sitter instead of implementing an AST parser for kotlin.
- [2026-08-12] Found a real parsing gap in the vendored `tree-sitter-kotlin` grammar while adding fixture-corpus tests: a multi-line generic type parameter list with a trailing comma before the closing `>` produces an ERROR node (see `src/interop/tree_sitter.rs`, `KNOWN_PARSE_ERROR_FIXTURES`). Not fixed yet; tracked as a known limitation instead of silently ignored, so diagnostics/completion may misbehave on code written in this style until the grammar is patched upstream or the fork is updated.
