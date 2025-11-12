# Plugin API

## Overview

bux supports plugins via the bux-sdk crate. Plugins can be:
- WASM modules (recommended for security)
- Native Rust modules (for performance)

## Plugin Interface

Plugins implement the `PluginApi` trait:

```rust
pub trait PluginApi {
    fn execute(&self, command: &str, args: &[Value]) -> Result<Value, String>;
}
```

## WASM Plugins

WASM plugins are compiled to WebAssembly and loaded at runtime.

### Creating a WASM Plugin

1. Use the template in `examples/plugin_template/`
2. Implement the plugin interface
3. Compile to WASM
4. Load in bux

### Example

See `examples/wasm_plugin_example.rs` for a complete example.

## Native Plugins

Native plugins are Rust crates that implement the PluginApi trait.

## Security

- WASM plugins run in a sandbox
- Native plugins have full system access
- Use WASM for untrusted plugins

