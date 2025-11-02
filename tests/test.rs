use glob_workflow_paths::match_pattern;

#[test]
fn test_basic_wildcards() {
    assert_glob_match("*", "README.md", true);
    assert_glob_match("*", "server.rb", true);
    assert_glob_match("*", "docs/file.txt", false);
}

#[test]
fn test_question_mark_wildcard() {
    // Test *.jsx? pattern - matches zero or one 'x'
    assert_glob_match("*.jsx?", "page.js", true); // matches - 'x' appears zero times
    assert_glob_match("*.jsx?", "page.jsx", true); // matches - 'x' appears one time
    assert_glob_match("*.jsx?", "page.jsxx", false); // doesn't match - 'x' appears more than once
    assert_glob_match("*.jsx?", "page.ts", false); // doesn't match - doesn't end with js/jsx
    assert_glob_match("*.jsx?", "component.js", true);
    assert_glob_match("*.jsx?", "component.jsx", true);
}

#[test]
fn test_double_star_wildcard() {
    // Test ** pattern - matches any character including slash (/)
    assert_glob_match("**", "all/the/files.md", true);
    assert_glob_match("**", "README.md", true);
    assert_glob_match("**", "docs/nested/deeply/file.txt", true);
    assert_glob_match("**", "single-file.js", true);
    assert_glob_match("**", "", true); // matches empty path
}

#[test]
fn test_js_extension_pattern() {
    // Test *.js pattern - matches all .js files at the root of the repository
    assert_glob_match("*.js", "app.js", true);
    assert_glob_match("*.js", "index.js", true);
    assert_glob_match("*.js", "main.js", true);
    assert_glob_match("*.js", "component.jsx", false); // wrong extension
    assert_glob_match("*.js", "src/app.js", false); // not at root (contains slash)
    assert_glob_match("*.js", "docs/script.js", false); // not at root (contains slash)
}

fn assert_glob_match(pattern: &str, path: &str, expected: bool) {
    let matches = match_pattern(pattern, path);

    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{}' -> {} (expected {})",
        pattern, path, matches, expected
    );
}
