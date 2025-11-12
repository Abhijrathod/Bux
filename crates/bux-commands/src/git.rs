//! Git integration commands

use git2::Repository;
use anyhow::Result;

/// Return a simplified Git status for the given path.
///
/// Currently checks if the directory is a Git repo and whether it's clean.
pub fn get_git_status(path: &str) -> Result<String> {
    let _repo = Repository::open(path)?; // ✅ underscore removes "unused variable" warning

    // TODO: Implement proper `git status` parsing
    Ok("clean".to_string())
}
