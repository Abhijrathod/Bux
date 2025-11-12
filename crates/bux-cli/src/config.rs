//! Configuration loader for the Bux CLI.
//! Supports aliases and environment variables via `~/.buxrc`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

/// Stores aliases and environment variables loaded from `~/.buxrc`
#[derive(Debug, Default)]
pub struct Config {
    pub aliases: HashMap<String, String>,
    pub env_vars: HashMap<String, String>,
}

impl Config {
    /// Loads configuration from `~/.buxrc`
    pub fn load() -> Self {
        let mut config = Config::default();

        if let Some(path) = get_config_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if line.starts_with("alias ") {
                        if let Some((name, value)) = parse_alias(line) {
                            config.aliases.insert(name, value);
                        }
                    } else if line.starts_with("set ") {
                        if let Some((key, value)) = parse_env(line) {
                            config.env_vars.insert(key.clone(), value.clone());
                            std::env::set_var(key, value);
                        }
                    }
                }
            }
        }

        config
    }

    /// Get alias by name
    pub fn alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    /// Get environment variable from config
    pub fn env(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }
}

/// Helper: Find `~/.buxrc` path
fn get_config_path() -> Option<PathBuf> {
    let mut path = home_dir()?;
    path.push(".buxrc");
    Some(path)
}

/// Parse alias lines like: alias ll="ls -l"
fn parse_alias(line: &str) -> Option<(String, String)> {
    let line = line.strip_prefix("alias ")?;
    let mut parts = line.splitn(2, '=');
    let name = parts.next()?.trim().to_string();
    let value = parts.next()?.trim().trim_matches('"').to_string();
    Some((name, value))
}

/// Parse env vars like: set PATH=/usr/bin
fn parse_env(line: &str) -> Option<(String, String)> {
    let line = line.strip_prefix("set ")?;
    let mut parts = line.splitn(2, '=');
    let key = parts.next()?.trim().to_string();
    let value = parts.next()?.trim().to_string();
    Some((key, value))
}
