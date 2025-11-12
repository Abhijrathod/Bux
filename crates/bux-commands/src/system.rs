//! System information commands

use std::io;

/// Return the system hostname as a String.
/// Falls back to "unknown" if hostname is unavailable.
pub fn get_hostname() -> String {
    hostname::get()
        .and_then(|h| {
            h.into_string()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid hostname"))
        })
        .unwrap_or_else(|_| "unknown".to_string())
}
