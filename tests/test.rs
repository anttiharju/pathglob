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

#[test]
fn test_double_star_js_extension_pattern() {
    // Test **.js pattern - matches all .js files in the repository
    assert_glob_match("**.js", "index.js", true);
    assert_glob_match("**.js", "js/index.js", true);
    assert_glob_match("**.js", "src/js/app.js", true);
    assert_glob_match("**.js", "deeply/nested/path/to/file.js", true);
    assert_glob_match("**.js", "component.jsx", false); // wrong extension
    assert_glob_match("**.js", "app.ts", false); // wrong extension
    assert_glob_match("**.js", "script.js.backup", false); // doesn't end with .js
}

#[test]
fn test_docs_directory_pattern() {
    // Test docs/* pattern - matches all files within the root of the docs directory only
    assert_glob_match("docs/*", "docs/README.md", true);
    assert_glob_match("docs/*", "docs/file.txt", true);
    assert_glob_match("docs/*", "docs/guide.md", true);
    assert_glob_match("docs/*", "docs/nested/file.txt", false); // nested files don't match
    assert_glob_match("docs/*", "README.md", false); // not in docs directory
    assert_glob_match("docs/*", "src/docs/file.txt", false); // docs not at root
    assert_glob_match("docs/*", "docs", false); // directory itself, not files within
}

#[test]
fn test_docs_recursive_pattern() {
    // Test docs/** pattern - matches any files in docs directory and its subdirectories
    assert_glob_match("docs/**", "docs/README.md", true);
    assert_glob_match("docs/**", "docs/mona/octocat.txt", true);
    assert_glob_match("docs/**", "docs/nested/deeply/file.txt", true);
    assert_glob_match("docs/**", "docs/guide.md", true);
    assert_glob_match("docs/**", "README.md", false); // not in docs directory
    assert_glob_match("docs/**", "src/docs/file.txt", false); // docs not at root
    assert_glob_match("docs/**", "other/docs/file.txt", false); // docs not at root
}

fn assert_glob_match(pattern: &str, path: &str, expected: bool) {
    let matches = match_pattern(pattern, path);

    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{}' -> {} (expected {})",
        pattern, path, matches, expected
    );
}
