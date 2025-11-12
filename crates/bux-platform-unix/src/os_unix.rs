//! Unix OS-specific functionality

#[cfg(unix)]
pub fn get_uid() -> u32 {
    unsafe { libc::getuid() }
}

