//! bux CLI - Main entry point for the Bux shell

mod config;
mod prompt;
mod history;
mod builtins;
mod alias;
mod theme;
mod utils;

use config::Config;
use rustyline::{Editor};
use rustyline::history::DefaultHistory;
use bux_core::executor::execute_command;
use prompt::{build_prompt, BuxPrompt};
use builtins::run_builtin;
use std::io::{self, Write};

fn main() {
    // Initialize readline editor with custom prompt
    let mut rl = Editor::<BuxPrompt, DefaultHistory>::new().unwrap();
    rl.set_helper(Some(BuxPrompt));

    println!("🪶 Welcome to bux shell — type 'exit' to quit\n");

    // 🔹 Load user config (~/.buxrc)
    let config = Config::load();
    if !config.aliases.is_empty() || !config.env_vars.is_empty() {
        println!(
            "Loaded {} aliases and {} env vars from ~/.buxrc\n",
            config.aliases.len(),
            config.env_vars.len()
        );
    }

    // 🔹 Shell loop
    loop {
        let prompt = build_prompt();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let line = rl.readline("");
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(trimmed);

                if trimmed == "exit" {
                    println!("Goodbye 👋");
                    break;
                }

                // 🔹 Check for alias expansion
                let expanded = expand_alias(trimmed, &config);

                // 🔹 Execute command
                if let Err(e) = run_command(&expanded) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                println!();
                break;
            }
        }
    }
}

/// Expand aliases defined in ~/.buxrc before executing
fn expand_alias(input: &str, config: &Config) -> String {
    let mut parts = input.split_whitespace();
    if let Some(cmd) = parts.next() {
        if let Some(alias_val) = config.alias(cmd) {
            let remaining = parts.collect::<Vec<&str>>().join(" ");
            if remaining.is_empty() {
                return alias_val.clone();
            } else {
                return format!("{} {}", alias_val, remaining);
            }
        }
    }
    input.to_string()
}

/// Run built-in or system command
fn run_command(input: &str) -> Result<(), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let cmd = parts[0];
    let args = &parts[1..];

    // 🔹 Check for built-in commands
    if run_builtin(cmd, args) {
        return Ok(());
    }

    // 🔹 Run external system commands
    match execute_command(cmd, args) {
        Ok(output) => {
            print!("{}", output);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
