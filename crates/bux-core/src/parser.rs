//! Command parser for bux shell

/// Token types for parsed commands
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
    RedirectOut,
    RedirectIn,
    AppendOut,
    Background,
}

/// Parse a command string into tokens
pub fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for ch in input.chars() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
                quote_char = '\0';
            }
            '|' if !in_quotes => {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.trim().to_string()));
                    current_word.clear();
                }
                tokens.push(Token::Pipe);
            }
            '>' if !in_quotes => {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.trim().to_string()));
                    current_word.clear();
                }
                tokens.push(Token::RedirectOut);
            }
            '<' if !in_quotes => {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.trim().to_string()));
                    current_word.clear();
                }
                tokens.push(Token::RedirectIn);
            }
            '&' if !in_quotes => {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.trim().to_string()));
                    current_word.clear();
                }
                tokens.push(Token::Background);
            }
            ' ' | '\t' if !in_quotes => {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.trim().to_string()));
                    current_word.clear();
                }
            }
            c => {
                current_word.push(c);
            }
        }
    }

    if !current_word.is_empty() {
        tokens.push(Token::Word(current_word.trim().to_string()));
    }

    tokens
}

/// Extract the first command and its arguments from tokens
pub fn extract_command(tokens: &[Token]) -> Option<(String, Vec<String>)> {
    let mut words: Vec<String> = tokens
        .iter()
        .take_while(|t| matches!(t, Token::Word(_)))
        .filter_map(|t| {
            if let Token::Word(w) = t {
                Some(w.clone())
            } else {
                None
            }
        })
        .collect();

    if words.is_empty() {
        return None;
    }

    let cmd = words.remove(0);
    Some((cmd, words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let tokens = parse("echo hello");
        assert_eq!(tokens, vec![Token::Word("echo".to_string()), Token::Word("hello".to_string())]);
    }

    #[test]
    fn test_parse_with_quotes() {
        let tokens = parse("echo \"hello world\"");
        assert_eq!(tokens, vec![Token::Word("echo".to_string()), Token::Word("hello world".to_string())]);
    }

    #[test]
    fn test_extract_command() {
        let tokens = parse("echo hello world");
        let (cmd, args) = extract_command(&tokens).unwrap();
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello", "world"]);
    }
}

