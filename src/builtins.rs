use std::env;

pub fn run_builtin(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "cd" => {
            let new_dir = args.get(0).map(|s| *s).unwrap_or(".");
            if let Err(e) = env::set_current_dir(new_dir) {
                eprintln!("cd: {}", e);
            }
            true
        }
        "pwd" => {
            if let Ok(path) = env::current_dir() {
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
