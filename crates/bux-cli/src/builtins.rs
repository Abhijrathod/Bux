//! Built-in shell commands

use bux_commands::filesystem;

pub fn run_builtin(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "cd" => {
            let path = args.get(0).map(|s| *s).unwrap_or(".");
            if let Err(e) = filesystem::cd(path) {
                eprintln!("cd: {}", e);
            }
            true
        }
        "pwd" => {
            if let Ok(path) = filesystem::pwd() {
                println!("{}", path.display());
            }
            true
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            true
        }
        _ => false,
    }
}

