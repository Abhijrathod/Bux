//! Shell runtime environment

use std::collections::HashMap;
use crate::value::Value;

/// Runtime environment for the shell
pub struct Runtime {
    variables: HashMap<String, Value>,
    aliases: HashMap<String, String>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set_alias(&mut self, name: String, value: String) {
        self.aliases.insert(name, value);
    }

    pub fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

