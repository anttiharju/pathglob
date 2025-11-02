pub fn match_pattern(patterns: &[&str], paths: &[&str]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let path = paths[0];
    let path_segments = if path.is_empty() { vec![] } else { path.split('/').collect() };

    // Check if there are any negation patterns
    let has_negations = patterns.iter().any(|p| p.starts_with('!'));

    if !has_negations {
        // Fast path: no negations, return true if ANY pattern matches
        for pattern in patterns {
            let parsed_pattern = parse_pattern(pattern);
            if match_segments(&parsed_pattern.segments, &path_segments, 0, 0) {
                return true;
            }
        }
        return false;
    }

    // Slow path: has negations, need to check all patterns
    let mut matched = false;

    for pattern in patterns {
        if pattern.starts_with('!') {
            // Negation pattern
            let negated_pattern = &pattern[1..];
            let parsed_pattern = parse_pattern(negated_pattern);
            if match_segments(&parsed_pattern.segments, &path_segments, 0, 0) {
                // If path matches a negation pattern, it's excluded
                return false;
            }
        } else {
            // Positive pattern
            let parsed_pattern = parse_pattern(pattern);
            if match_segments(&parsed_pattern.segments, &path_segments, 0, 0) {
                matched = true;
                // Don't return immediately - continue checking for negation patterns
            }
        }
    }

    matched
}

#[derive(Debug, Clone)]
struct Pattern {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
enum Segment {
    Literal(String),              // "docs", "file.txt"
    Star(String),                 // "*.js" -> Star(".js")
    DoubleStar,                   // "**"
    DoubleStarWithSuffix(String), // "**.js" -> DoubleStarWithSuffix(".js")
    Optional(String, char),       // "*.jsx?" -> Optional(".js", 'x')
}

fn parse_pattern(pattern: &str) -> Pattern {
    let parts: Vec<&str> = pattern.split('/').collect();
    let mut segments = Vec::new();

    for part in parts {
        if part == "**" {
            segments.push(Segment::DoubleStar);
        } else if part.starts_with("**") {
            // Handle **.js patterns
            let suffix = &part[2..];
            segments.push(Segment::DoubleStarWithSuffix(suffix.to_string()));
        } else if part.contains('*') && part.contains('?') {
            // Handle patterns like *.jsx?
            if let Some(question_pos) = part.rfind('?') {
                if question_pos > 0 {
                    let before_question = &part[..question_pos];
                    let optional_char = part.chars().nth(question_pos - 1).unwrap();
                    let without_optional =
                        format!("{}{}", &before_question[..before_question.len() - 1], &part[question_pos + 1..]);
                    segments.push(Segment::Optional(without_optional, optional_char));
                } else {
                    // Fallback to star pattern
                    segments.push(Segment::Star(part.to_string()));
                }
            } else {
                segments.push(Segment::Star(part.to_string()));
            }
        } else if part.contains('*') {
            segments.push(Segment::Star(part.to_string()));
        } else {
            segments.push(Segment::Literal(part.to_string()));
        }
    }

    Pattern { segments }
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

        Segment::Optional(base_pattern, optional_char) => {
            if path_idx >= path_parts.len() {
                return false;
            }

            // Try with the optional character (e.g., "*.jsx" for "*.jsx?")
            let with_optional = format!("{}{}", &base_pattern[..base_pattern.len()], optional_char);
            if matches_star_pattern(&with_optional, path_parts[path_idx]) {
                return match_segments(segments, path_parts, seg_idx + 1, path_idx + 1);
            }

            // Try without the optional character (e.g., "*.js" for "*.jsx?")
            if matches_star_pattern(base_pattern, path_parts[path_idx]) {
                return match_segments(segments, path_parts, seg_idx + 1, path_idx + 1);
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
