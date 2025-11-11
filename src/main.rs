mod command;
mod builtins;
mod prompt;
mod history;

use rustyline::Editor;
use command::run_command;
use prompt::build_prompt;

fn main() {
    let mut rl = Editor::<(), _>::new().unwrap();

    println!("🪶 Welcome to bux shell — type 'exit' to quit\n");

    loop {
        let prompt = build_prompt();
        let line = rl.readline(&prompt);

        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() { continue; }

                rl.add_history_entry(trimmed);

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
