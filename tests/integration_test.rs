use glob_workflow_paths::match_pattern;

#[test]
fn test_basic_wildcards() {
    assert_glob_match("*", "README.md", true);
    assert_glob_match("*", "server.rb", true);
    assert_glob_match("*", "docs/file.txt", false);
}

fn assert_glob_match(pattern: &str, path: &str, expected: bool) {
    let matches = match_pattern(pattern, path);

    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{}' -> {} (expected {})",
        pattern, path, matches, expected
    );
}
