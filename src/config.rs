use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

/// Stores aliases and environment vars loaded from ~/.buxrc
#[derive(Debug, Default)]
pub struct Config {
    pub aliases: HashMap<String, String>,
    pub env_vars: HashMap<String, String>,
}

impl Config {
    /// Loads ~/.buxrc and parses aliases + env vars
    pub fn load() -> Self {
        let mut config = Config::default();

        if let Some(path) = get_config_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    let line = line.trim();

                    // Skip comments and empty lines
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if line.starts_with("alias ") {
                        // alias name="command"
                        if let Some((name, value)) = parse_alias(line) {
                            config.aliases.insert(name, value);
                        }
                    } else if line.starts_with("set ") {
                        // set VAR=value
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
}

/// Helper: Find ~/.buxrc path
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
