pub fn match_pattern(pattern: &str, path: &str) -> bool {
    match_single_pattern(pattern, path)
}

fn match_single_pattern(pattern: &str, path: &str) -> bool {
    // Convert pattern to regex-like tokens for easier processing
    let tokens = tokenize_pattern(pattern);
    match_tokens(&tokens, path.chars().collect(), 0)
}

#[derive(Debug, Clone)]
enum Token {
    Literal(char),
    Star,           // * - matches any chars except /
    DoubleStar,     // ** - matches any chars including /
    Optional(char), // x? - matches zero or one x
}

fn tokenize_pattern(pattern: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    // Don't check for **/* pattern here - just create DoubleStar
                    tokens.push(Token::DoubleStar);
                    i += 2;
                } else {
                    tokens.push(Token::Star);
                    i += 1;
                }
            }
            '?' => {
                if i > 0 {
                    // Convert previous literal + ? into Optional
                    if let Some(Token::Literal(c)) = tokens.pop() {
                        tokens.push(Token::Optional(c));
                    }
                }
                i += 1;
            }
            c => {
                tokens.push(Token::Literal(c));
                i += 1;
            }
        }
    }

    tokens
}

fn match_tokens(tokens: &[Token], path: Vec<char>, path_idx: usize) -> bool {
    if tokens.is_empty() && path_idx >= path.len() {
        return true;
    }

    if tokens.is_empty() {
        return false;
    }

    match &tokens[0] {
        Token::Literal(c) => {
            if path_idx >= path.len() || path[path_idx] != *c {
                return false;
            }
            match_tokens(&tokens[1..], path, path_idx + 1)
        }
        Token::Star => {
            // Try matching zero characters first
            if match_tokens(&tokens[1..], path.clone(), path_idx) {
                return true;
            }

            // Try matching one or more characters (but not '/')
            let mut i = path_idx;
            while i < path.len() && path[i] != '/' {
                i += 1;
                if match_tokens(&tokens[1..], path.clone(), i) {
                    return true;
                }
            }
            false
        }
        Token::DoubleStar => {
            // Handle ** followed by / specially
            if tokens.len() > 1 && matches!(tokens[1], Token::Literal('/')) {
                // This is **/ pattern
                // Try matching zero directories (stay at current position and skip **/)
                if match_tokens(&tokens[2..], path.clone(), path_idx) {
                    return true;
                }

                // Try matching one or more path segments
                for i in path_idx..path.len() {
                    if path[i] == '/' {
                        // Found a slash, try matching from the position after it
                        if match_tokens(&tokens[2..], path.clone(), i + 1) {
                            return true;
                        }
                    }
                }
                false
            } else {
                // Regular ** without following /
                // Try zero-length match first
                if match_tokens(&tokens[1..], path.clone(), path_idx) {
                    return true;
                }

                // Try consuming characters one by one
                for i in (path_idx + 1)..=path.len() {
                    if match_tokens(&tokens[1..], path.clone(), i) {
                        return true;
                    }
                }
                false
            }
        }
        Token::Optional(c) => {
            // Try zero occurrences
            if match_tokens(&tokens[1..], path.clone(), path_idx) {
                return true;
            }

            // Try one occurrence
            if path_idx < path.len() && path[path_idx] == *c {
                return match_tokens(&tokens[1..], path, path_idx + 1);
            }

            false
        }
    }
}
