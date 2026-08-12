use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_kotlin() -> Language; }

pub fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language( unsafe { tree_sitter_kotlin() }).expect("Error loading Kotlin grammar.");
    parser
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{load_corpus_fixtures, load_gasoline_fixtures};

    // The `tree-sitter` crate version pinned in Cargo.toml and the grammar ABI
    // baked into vendor/tree-sitter-kotlin's generated parser.c can drift apart
    // (see docs/ADR.md history) — set_language() panics on ABI mismatch. This
    // test is a fast, explicit trip-wire for that: if it starts failing after a
    // submodule bump or a `tree-sitter` version bump, that is the cause.
    #[test]
    fn get_parser_does_not_panic() {
        let _ = get_parser();
    }

    // Smoke-tests the parser against the grammar's own corpus. This does not
    // check exact tree shape (that would require the `tree-sitter test` CLI) —
    // only that every canonical Kotlin construct the grammar claims to support
    // parses without an ERROR node.
    #[test]
    fn parses_vendor_corpus_without_errors() {
        let fixtures = load_corpus_fixtures();
        assert!(!fixtures.is_empty(), "expected corpus fixtures to be non-empty");

        let mut parser = get_parser();
        let mut failures = Vec::new();

        for (name, source) in fixtures {
            let tree = parser.parse(&source, None).expect("parse should not return None for string input");
            if tree.root_node().kind() != "source_file" {
                failures.push(format!("{name}: unexpected root kind {}", tree.root_node().kind()));
            } else if tree.root_node().has_error() {
                failures.push(format!("{name}: parse tree contains an ERROR node"));
            }
        }

        assert!(failures.is_empty(), "corpus fixtures failed to parse cleanly:\n{}", failures.join("\n"));
    }

    // Fixtures known to trip a real grammar limitation: a multi-line generic
    // type parameter list with a trailing comma before the closing `>`, e.g.
    //
    //   private inline fun <
    //       reified predicatedAnnotation : Any,
    //   > findFirstMethodAnnotated(testClass: Any) = ...
    //
    // Found 2026-08-12 while adding this test — see docs/ADR.md. This is a
    // quarantine list, not a way to hide failures: it must shrink as the
    // grammar improves, and grow only with a matching ADR entry.
    const KNOWN_PARSE_ERROR_FIXTURES: &[&str] = &["TestRunner", "TestValidator"];

    // Smoke-tests the parser against real, hand-written Kotlin code (vendored
    // from the gasoline project) rather than the grammar's own minimal corpus —
    // catches regressions the corpus alone would miss. Fixtures on the known-
    // limitation list above are expected to fail; everything else must pass.
    #[test]
    fn parses_gasoline_fixtures_without_errors() {
        let fixtures = load_gasoline_fixtures();
        assert!(!fixtures.is_empty(), "expected gasoline fixtures to be non-empty");

        let mut parser = get_parser();
        let mut unexpected_failures = Vec::new();
        let mut unexpected_passes = Vec::new();

        for (name, source) in fixtures {
            let tree = parser.parse(&source, None).expect("parse should not return None for string input");
            let has_error =
                tree.root_node().kind() != "source_file" || tree.root_node().has_error();
            let is_known_failure = KNOWN_PARSE_ERROR_FIXTURES.contains(&name.as_str());

            match (has_error, is_known_failure) {
                (true, false) => unexpected_failures.push(name),
                (false, true) => unexpected_passes.push(name),
                _ => {}
            }
        }

        assert!(
            unexpected_failures.is_empty(),
            "these fixtures newly fail to parse cleanly (regression):\n{}",
            unexpected_failures.join("\n")
        );
        assert!(
            unexpected_passes.is_empty(),
            "these fixtures now parse cleanly — remove them from KNOWN_PARSE_ERROR_FIXTURES:\n{}",
            unexpected_passes.join("\n")
        );
    }
}
