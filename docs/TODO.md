# TODO

- [x] Set up automated tests for the server.
- [x] Integrate `clippy` into the CI pipeline to enforce code quality.
- [ ] Implement diagnostics (error squiggles) by detecting errors in the syntax tree.
- [ ] Add syntax highlighting using the `tree-sitter-highlight` crate.
- [ ] Fix `get_code_context` (`src/handlers/completion.rs`) treating the LSP line number as a byte offset — panics on any completion request past a short document's byte length. See the test `line_number_used_as_byte_offset_panics_on_short_documents`.
- [ ] Fix the vendored `tree-sitter-kotlin` grammar's ERROR on multi-line generic type parameter lists with a trailing comma. See `docs/ADR.md` (2026-08-12) and `KNOWN_PARSE_ERROR_FIXTURES` in `src/interop/tree_sitter.rs`.
