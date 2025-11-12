use std::collections::HashMap;

/// Manages shell aliases
#[derive(Debug, Default)]
pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    /// Create a new alias manager
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    /// Load aliases from a config
    pub fn load_aliases(&mut self, aliases: HashMap<String, String>) {
        self.aliases.extend(aliases);
    }

    /// Add or update an alias
    pub fn set_alias(&mut self, name: String, value: String) {
        self.aliases.insert(name, value);
    }

    /// Remove an alias
    pub fn unset_alias(&mut self, name: &str) -> bool {
        self.aliases.remove(name).is_some()
    }

    /// Get an alias value
    pub fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    /// List all aliases
    pub fn list_aliases(&self) -> &HashMap<String, String> {
        &self.aliases
    }

    /// Expand aliases in a command string
    pub fn expand(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return input.to_string();
        }

        if let Some(alias_value) = self.get_alias(parts[0]) {
            let mut expanded = alias_value.clone();
            if parts.len() > 1 {
                expanded.push(' ');
                expanded.push_str(&parts[1..].join(" "));
            }
            expanded
        } else {
            input.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_alias() {
        let mut mgr = AliasManager::new();
        mgr.set_alias("ll".to_string(), "ls -l".to_string());
        assert_eq!(mgr.get_alias("ll"), Some(&"ls -l".to_string()));
    }

    #[test]
    fn test_expand_alias() {
        let mut mgr = AliasManager::new();
        mgr.set_alias("ll".to_string(), "ls -l".to_string());
        assert_eq!(mgr.expand("ll"), "ls -l");
        assert_eq!(mgr.expand("ll -a"), "ls -l -a");
    }

    #[test]
    fn test_unset_alias() {
        let mut mgr = AliasManager::new();
        mgr.set_alias("ll".to_string(), "ls -l".to_string());
        assert!(mgr.unset_alias("ll"));
        assert_eq!(mgr.get_alias("ll"), None);
    }
}

