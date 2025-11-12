//! bux-platform-win: Windows-specific platform code
//! Similar to powershell-win-core

#[cfg(windows)]
pub mod os_windows;
#[cfg(windows)]
pub mod registry;

#[cfg(windows)]
pub use os_windows::*;
#[cfg(windows)]
pub use registry::*;

