use colored::*;
use std::env;

pub fn build_prompt() -> String {
    let cwd = env::current_dir()
        .unwrap_or_else(|_| "unknown".into())
        .display()
        .to_string();

    format!(
        "{} {} > ",
        "bux".bright_cyan().bold(),
        cwd.bright_green()
    )
}
