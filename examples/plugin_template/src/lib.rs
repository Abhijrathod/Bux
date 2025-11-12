//! Template for creating a bux WASM plugin
//! 
//! To build:
//! 1. rustup target add wasm32-unknown-unknown
//! 2. cargo build --target wasm32-unknown-unknown --release
//! 3. wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/bux_plugin_template.wasm

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Plugin request structure
#[derive(Serialize, Deserialize)]
pub struct PluginRequest {
    pub command: String,
    pub args: Vec<String>,
}

/// Plugin response structure
#[derive(Serialize, Deserialize)]
pub struct PluginResponse {
    pub output: String,
    pub exit_code: i32,
}

/// Main plugin entry point
#[wasm_bindgen]
pub fn execute(command: &str, input: &str) -> String {
    // Parse the command
    let request: PluginRequest = serde_json::from_str(input)
        .unwrap_or_else(|_| PluginRequest {
            command: command.to_string(),
            args: vec![],
        });

    // Execute the plugin logic
    let response = PluginResponse {
        output: format!("Plugin executed: {}", request.command),
        exit_code: 0,
    };

    // Return JSON response
    serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! This is a bux plugin.", name));
}

