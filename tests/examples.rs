use aleph_syntax_tree::syntax::AlephTree;
use std::fs;
use std::path::Path;

/// Every `.ale` file under `test/dataset/ale/` must parse successfully —
/// this is the permanent, automated form of a manual smoke-test that once
/// caught a real regression (the 0.2 typed-function `->` token silently
/// breaking the pre-existing `match ... -> ...:` arrow syntax) that no
/// other test in this crate exercised. `parse()` doesn't propagate a
/// `Result` (it prints and returns `AlephTree::Unit` on failure), so a
/// parse failure here shows up as an unexpected `Unit` for a file whose
/// content is never just `Unit` on its own.
#[test]
fn every_example_file_parses_successfully() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/dataset/ale");
    let mut failures = Vec::new();
    let mut count = 0;

    let mut dirs = vec![root];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).expect("dataset dir should exist") {
            let path = entry.expect("readable dir entry").path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if path.extension().map(|e| e == "ale").unwrap_or(false) {
                count += 1;
                let source = fs::read_to_string(&path).expect("readable .ale file");
                let tree = aleparser::parse(source);
                if tree == AlephTree::Unit {
                    failures.push(path);
                }
            }
        }
    }

    assert!(count > 0, "expected at least one .ale example file under test/dataset/ale");
    assert!(
        failures.is_empty(),
        "{} of {} example file(s) failed to parse: {:?}",
        failures.len(), count, failures
    );
}
