# bux WASM Plugin Template

This is a template for creating WASM plugins for bux.

## Building a Plugin

1. Copy this template directory:
   ```bash
   cp -r examples/plugin_template my_plugin
   cd my_plugin
   ```

2. Update `Cargo.toml` with your plugin name and dependencies

3. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. Build your plugin:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

5. Use `wasm-bindgen` to generate bindings:
   ```bash
   wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/my_plugin.wasm
   ```

6. Load the plugin in bux (when plugin loading is implemented)

## Plugin Interface

Plugins should export an `execute` function that takes:
- `command: &str` - The command to execute
- `input: &str` - JSON-encoded input data

And returns a JSON-encoded `PluginResponse`:
```json
{
  "output": "Plugin output text",
  "exit_code": 0
}
```

## Example

See `src/lib.rs` for a complete example implementation.

