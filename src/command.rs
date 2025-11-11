use std::process::Command;
use crate::builtins::run_builtin;

pub fn run_command(input: &str) -> Result<(), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() { return Ok(()); }

    let cmd = parts[0];
    let args = &parts[1..];

    // Check for built-in commands first
    if run_builtin(cmd, args) {
        return Ok(());
    }

    // Run external system commands
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
