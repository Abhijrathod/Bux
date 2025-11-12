//! bux-native: Native system calls and FFI
//! Similar to libpsl-native

pub mod ffi;
pub mod syscalls;

pub use ffi::*;
pub use syscalls::*;

