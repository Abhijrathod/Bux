//! Value types for bux shell (similar to PSObject in PowerShell)

use serde::{Deserialize, Serialize};

/// Shell value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(serde_json::Map<String, serde_json::Value>),
    Null,
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => String::new(),
            Value::Array(_) | Value::Object(_) => format!("{:?}", self),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

