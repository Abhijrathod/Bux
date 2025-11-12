//! Parser tests

#[cfg(test)]
mod tests {
    use bux_core::parser;

    #[test]
    fn test_parse_commands() {
        let tokens = parser::parse("echo hello");
        assert!(!tokens.is_empty());
    }
}

