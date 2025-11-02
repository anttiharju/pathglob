pub fn match_pattern(pattern: &str, path: &str) -> bool {
    pattern == "*" && !path.contains('/')
}
