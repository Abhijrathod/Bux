//! Example WASM plugin for bux
//! 
//! This example demonstrates how to create and use a WASM plugin in bux.
//! 
//! To build a WASM plugin:
//! 1. Create a new Rust project with `cargo new --lib my_plugin`
//! 2. Add `wasm32-wasi` target: `rustup target add wasm32-wasi`
//! 3. Configure Cargo.toml for WASM:
//!    [lib]
//!    crate-type = ["cdylib"]
//!    
//!    [dependencies]
//!    wasm-bindgen = "0.2"
//! 4. Build: `cargo build --target wasm32-wasi --release`
//! 5. Load the .wasm file in bux

use anyhow::Result;
use std::fs;

#[cfg(feature = "wasm")]
use crate::plugin::{PluginManager, WasmPlugin};

/// Example: Load and use a WASM plugin
#[cfg(feature = "wasm")]
pub fn example_load_wasm_plugin() -> Result<()> {
    let mut manager = crate::plugin::PluginManager::new();
    
    // Load WASM plugin from file
    let wasm_bytes = fs::read("examples/example_plugin.wasm")?;
    let plugin = WasmPlugin::new("example".to_string(), &wasm_bytes)?;
    
    manager.register(Box::new(plugin));
    
    // Execute plugin
    if let Some(plugin) = manager.find_plugin("example") {
        let result = plugin.execute("test_command", b"input data")?;
        println!("Plugin output: {}", result.output);
    }
    
    Ok(())
}

/// Example plugin contract (what plugins should implement)
/// 
/// Plugins should export functions that match this interface:
/// - `execute(command: &str, input: &[u8]) -> PluginResult`
/// 
/// Communication is done via JSON strings for portability.
pub mod plugin_contract {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct PluginRequest {
        pub command: String,
        pub input: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PluginResponse {
        pub output: String,
        pub exit_code: i32,
    }
}

#[cfg(not(feature = "wasm"))]
pub fn example_load_wasm_plugin() -> Result<()> {
    println!("WASM feature not enabled. Build with --features wasm to use WASM plugins.");
    Ok(())
}

