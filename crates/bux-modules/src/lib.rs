//! bux-modules: Native built-in modules
//! Similar to PowerShell modules

pub mod sysinfo;
pub mod package;

pub use sysinfo::*;
pub use package::*;

