//! Process management commands

use std::process::Command;
use anyhow::Result;

pub fn run_process(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !stderr.is_empty() {
        eprint!("{}", stderr);
    }
    
    Ok(stdout.to_string())
}

