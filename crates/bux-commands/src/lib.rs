//! bux-commands: Built-in commands for the Bux shell.
//! Mirrors PowerShell's `Microsoft.PowerShell.Commands.*` modules.

pub mod filesystem;
pub mod process;
pub mod system;
pub mod git;

// 🔹 `network` module commented out until implemented
// pub mod network;

pub use filesystem::*;
pub use process::*;
pub use system::*;
pub use git::*;

// (network removed for now to avoid unused import warning)
