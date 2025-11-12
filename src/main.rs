mod command;
mod builtins;
mod prompt;
mod history;
mod alias;
mod parser;
mod plugin;
mod theme;
mod utils;
mod config;

use rustyline::{Editor};
use rustyline::history::DefaultHistory;
use command::run_command;
use prompt::{build_prompt, BuxPrompt};

fn main() {
    let mut rl = Editor::<BuxPrompt, DefaultHistory>::new().unwrap();
    rl.set_helper(Some(BuxPrompt));

    println!("🪶 Welcome to bux shell — type 'exit' to quit\n");

    loop {
        let prompt = build_prompt();
        print!("{}", prompt);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
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

                if let Err(e) = run_command(trimmed) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => break,
        }
    }
}
