//! bux-platform-unix: Unix-specific platform code
//! Similar to powershell-unix

#[cfg(unix)]
pub mod os_unix;
#[cfg(unix)]
pub mod process;

#[cfg(unix)]
pub use os_unix::*;
#[cfg(unix)]
pub use process::*;

