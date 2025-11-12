/// Theme management for bux prompt and output

use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Color scheme for the shell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub prompt_bux_color: String,
    pub prompt_path_color: String,
    pub prompt_arrow_color: String,
    pub error_color: String,
    pub success_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            prompt_bux_color: "cyan".to_string(),
            prompt_path_color: "green".to_string(),
            prompt_arrow_color: "yellow".to_string(),
            error_color: "red".to_string(),
            success_color: "green".to_string(),
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), Theme::default());
        
        Self {
            themes,
            current_theme: "default".to_string(),
        }
    }

    pub fn load_theme(&mut self, name: String, theme: Theme) {
        self.themes.insert(name.clone(), theme);
    }

    pub fn set_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn get_theme(&self) -> &Theme {
        self.themes.get(&self.current_theme).unwrap_or_else(|| {
            self.themes.get("default").unwrap()
        })
    }

    pub fn list_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }
}

/// Apply color to text based on theme color name
pub fn apply_color(text: &str, color_name: &str) -> ColoredString {
    match color_name.to_lowercase().as_str() {
        "black" => text.black(),
        "red" => text.red(),
        "green" => text.green(),
        "yellow" => text.yellow(),
        "blue" => text.blue(),
        "magenta" => text.magenta(),
        "cyan" => text.cyan(),
        "white" => text.white(),
        "bright_black" => text.bright_black(),
        "bright_red" => text.bright_red(),
        "bright_green" => text.bright_green(),
        "bright_yellow" => text.bright_yellow(),
        "bright_blue" => text.bright_blue(),
        "bright_magenta" => text.bright_magenta(),
        "bright_cyan" => text.bright_cyan(),
        "bright_white" => text.bright_white(),
        _ => text.normal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager() {
        let mgr = ThemeManager::new();
        assert_eq!(mgr.list_themes(), vec!["default"]);
    }

    #[test]
    fn test_apply_color() {
        let colored = apply_color("test", "red");
        // Just verify it doesn't panic
        assert!(!colored.to_string().is_empty());
    }
}

