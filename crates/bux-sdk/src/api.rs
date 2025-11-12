//! Public API for bux plugins

use bux_core::value::Value;

/// Plugin API trait
pub trait PluginApi {
    fn execute(&self, command: &str, args: &[Value]) -> Result<Value, String>;
}

