use glob_workflow_paths::match_paths;

fn assert_glob_match(patterns: &[&str], paths: &[&str], expected: bool) {
    let matches = match_paths(patterns, paths);
    assert_eq!(matches, expected, "Patterns '{:?}' vs '{:?}' -> {} (expected {})", patterns, paths, matches, expected);
}

#[test]
fn test_single_star_behavior() {
    // Matches zero or more characters
    assert_glob_match(&["Octo*"], &["Octocat"], true);
    assert_glob_match(&["Octo*"], &["Octo"], true); // zero characters

    // Does NOT match slash character
    assert_glob_match(&["Octo*"], &["Octo/cat"], false);
    assert_glob_match(&["*.js"], &["dir/app.js"], false);
}

#[test]
fn test_double_star_behavior() {
    // Matches zero or more of any character (including /)
    assert_glob_match(&["**"], &["anything"], true);
    assert_glob_match(&["**"], &["dir/file.txt"], true);
    assert_glob_match(&["**"], &["deep/nested/path/file.js"], true);
    assert_glob_match(&["**"], &[""], true); // zero characters

    // Compare with single star behavior
    assert_glob_match(&["*"], &["dir/file.txt"], false); // single * cannot match /
    assert_glob_match(&["**"], &["dir/file.txt"], true); // double ** can match /
}

#[test]
fn test_question_mark_behavior() {
    // Matches zero or one of the preceding character
    assert_glob_match(&["*.jsx?"], &["component.js"], true); // zero 'x'
    assert_glob_match(&["*.jsx?"], &["component.jsx"], true); // one 'x'
    assert_glob_match(&["*.jsx?"], &["component.jsxx"], false); // more than one 'x'

    // Works with other characters too
    assert_glob_match(&["file?.txt"], &["fil.txt"], true); // zero e characters
    assert_glob_match(&["file?.txt"], &["file.txt"], true); // one e character
    assert_glob_match(&["file?.txt"], &["filee.txt"], false); // two e
}
