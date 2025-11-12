//! Prompt rendering for bux CLI

use colored::*;
use std::env;
use std::borrow::Cow;
use rustyline::{Helper, highlight::Highlighter, hint::Hinter, validate::Validator, completion::Completer};

pub struct BuxPrompt;

impl Helper for BuxPrompt {}

impl Highlighter for BuxPrompt {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'p, str> {
        Cow::Borrowed(prompt)
    }
}

impl Hinter for BuxPrompt {
    type Hint = String;
}

impl Validator for BuxPrompt {}
impl Completer for BuxPrompt {
    type Candidate = String;
}

pub fn build_prompt() -> String {
    let cwd = env::current_dir()
        .unwrap_or_else(|_| "unknown".into())
        .display()
        .to_string();

    // Build the colored prompt without invisible markers since we print it manually
    let bux_colored = format!("{}", "🪶 bux".bright_cyan().bold());
    let path_colored = format!("{}", cwd.bright_green());
    let arrow_colored = format!("{}", ">".bright_yellow());

    format!("{bux_colored} {path_colored} {arrow_colored} ")
}

