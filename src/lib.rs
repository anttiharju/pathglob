pub fn match_paths(patterns: &[&str], paths: &[&str]) -> bool {
    if patterns.is_empty() || paths.is_empty() {
        return false;
    }

    // Pre-parse all patterns, expanding optionals into multiple variants
    let parsed_patterns: Vec<(Pattern, bool)> = patterns.iter().flat_map(|pattern| parse_pattern(pattern)).collect();

    // Check if any path matches the sequential pattern logic
    paths.iter().any(|path| {
        let path_segments = if path.is_empty() { vec![] } else { path.split('/').collect() };

        // Process pre-parsed patterns sequentially - each pattern can override previous results
        let mut matched = false;

        for (parsed_pattern, is_negation) in &parsed_patterns {
            if match_segments(&parsed_pattern.segments, &path_segments, 0, 0) {
                if *is_negation {
                    matched = false; // Negation pattern matched - exclude the path
                } else {
                    matched = true; // Positive pattern matched - include the path
                }
            }
        }

        matched
    })
}

#[derive(Debug, Clone)]
struct Pattern {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
enum Segment {
    Literal(String),              // "docs", "file.txt"
    Star(String),                 // "*.js" -> Star("*.js")
    DoubleStar,                   // "**"
    DoubleStarWithSuffix(String), // "**.js" -> DoubleStarWithSuffix(".js")
}

fn parse_pattern(pattern: &str) -> Vec<(Pattern, bool)> {
    let (actual_pattern, is_negation) = if pattern.starts_with('!') { (&pattern[1..], true) } else { (pattern, false) };

    // Expand optionals into multiple patterns
    expand_optionals(actual_pattern)
        .into_iter()
        .map(|expanded_pattern| {
            let parts: Vec<&str> = expanded_pattern.split('/').collect();
            let mut segments = Vec::new();

            for part in parts {
                if part == "**" {
                    segments.push(Segment::DoubleStar);
                } else if part.starts_with("**") {
                    let suffix = &part[2..];
                    segments.push(Segment::DoubleStarWithSuffix(suffix.to_string()));
                } else if part.contains('*') {
                    segments.push(Segment::Star(part.to_string()));
                } else {
                    segments.push(Segment::Literal(part.to_string()));
                }
            }

            (Pattern { segments }, is_negation)
        })
        .collect()
}

fn expand_optionals(pattern: &str) -> Vec<String> {
    if let Some(question_pos) = pattern.find('?') {
        if question_pos == 0 {
            return vec![pattern.to_string()]; // Invalid pattern, return as-is
        }

        let optional_char = pattern.chars().nth(question_pos - 1).unwrap();
        let before_optional = &pattern[..question_pos - 1];
        let after_optional = &pattern[question_pos + 1..];

        // Create two variants: without and with the optional character
        let pattern_without = format!("{}{}", before_optional, after_optional);
        let pattern_with = format!("{}{}{}", before_optional, optional_char, after_optional);

        // Recursively expand any remaining optionals in both variants
        let mut results = Vec::new();
        results.extend(expand_optionals(&pattern_without));
        results.extend(expand_optionals(&pattern_with));
        results
    } else {
        // No more optionals, return the pattern as-is
        vec![pattern.to_string()]
    }
}

fn match_segments(segments: &[Segment], path_parts: &[&str], seg_idx: usize, path_idx: usize) -> bool {
    // Base case: both exhausted
    if seg_idx >= segments.len() && path_idx >= path_parts.len() {
        return true;
    }

    // Pattern exhausted but path remains
    if seg_idx >= segments.len() {
        return false;
    }

    match &segments[seg_idx] {
        Segment::Literal(literal) => {
            if path_idx >= path_parts.len() || path_parts[path_idx] != literal {
                return false;
            }
            match_segments(segments, path_parts, seg_idx + 1, path_idx + 1)
        }

        Segment::Star(pattern) => {
            if path_idx >= path_parts.len() {
                return false;
            }

            if matches_star_pattern(pattern, path_parts[path_idx]) {
                match_segments(segments, path_parts, seg_idx + 1, path_idx + 1)
            } else {
                false
            }
        }

        Segment::DoubleStar => {
            // Try consuming 0 or more path segments
            // First try consuming 0 segments
            if match_segments(segments, path_parts, seg_idx + 1, path_idx) {
                return true;
            }

            // Then try consuming 1, 2, 3... segments
            for i in (path_idx + 1)..=path_parts.len() {
                if match_segments(segments, path_parts, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }

        Segment::DoubleStarWithSuffix(suffix) => {
            // **.js matches any file ending with .js anywhere in the repository
            for i in path_idx..path_parts.len() {
                if path_parts[i].ends_with(suffix) {
                    if match_segments(segments, path_parts, seg_idx + 1, i + 1) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn matches_star_pattern(pattern: &str, segment: &str) -> bool {
    if !pattern.contains('*') {
        return pattern == segment;
    }

    // Handle patterns like "*.js", "*README*", etc.
    let parts: Vec<&str> = pattern.split('*').collect();

    if parts.len() == 2 {
        let prefix = parts[0];
        let suffix = parts[1];

        segment.starts_with(prefix) && segment.ends_with(suffix) && segment.len() >= prefix.len() + suffix.len()
    } else if parts.len() == 1 {
        // Just "*"
        true
    } else {
        // More complex patterns with multiple *
        // For now, implement simple case
        // TODO: Handle patterns like "*foo*bar*"
        true
    }
}
