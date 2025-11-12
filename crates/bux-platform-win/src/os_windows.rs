//! Windows OS-specific functionality

#[cfg(windows)]
pub fn get_username() -> String {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
}

