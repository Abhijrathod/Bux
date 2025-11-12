/// Utility functions for bux

use std::path::PathBuf;

/// Expand `~` to home directory in a path
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path == "~" {
                return home;
            }
            let rest = path.strip_prefix("~/").unwrap_or(&path[1..]);
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}

/// Check if a string is a valid identifier (for variable/alias names)
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // First char must be letter or underscore
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Split a command string while respecting quotes
pub fn split_respecting_quotes(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for ch in s.chars() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
                quote_char = '\0';
                current.push(ch);
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current);
                    current = String::new();
                }
            }
            c => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("abc"));
        assert!(is_valid_identifier("_abc"));
        assert!(is_valid_identifier("abc123"));
        assert!(!is_valid_identifier("123abc"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("abc-def"));
    }

    #[test]
    fn test_split_respecting_quotes() {
        assert_eq!(
            split_respecting_quotes("echo hello"),
            vec!["echo", "hello"]
        );
        assert_eq!(
            split_respecting_quotes("echo \"hello world\""),
            vec!["echo", "\"hello world\""]
        );
    }
}

