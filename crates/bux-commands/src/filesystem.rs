//! Filesystem commands (cd, ls, pwd, etc.)

use std::env;
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use dirs; // make sure dirs is in Cargo.toml for this crate

/// Change directory (like `cd` command)
pub fn cd(path: &str) -> Result<()> {
    let target = if path.is_empty() {
        dirs::home_dir().ok_or_else(|| anyhow!("No home directory found"))?
    } else {
        PathBuf::from(path)
    };

    env::set_current_dir(&target)?;
    Ok(())
}

/// Print working directory (like `pwd`)
pub fn pwd() -> Result<PathBuf> {
    env::current_dir().map_err(Into::into)
}
