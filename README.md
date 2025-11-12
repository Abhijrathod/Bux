# bux

A modern, extensible, fast, cross-platform shell built in Rust.

## Features

- 🪶 **Cross-platform**: Great Windows support with cmd/PowerShell interop, first-class Linux/macOS behavior
- 🔌 **Extensible**: WASM-based plugin system for secure third-party extensions
- 🎨 **Customizable**: Rich prompt with Git info, themes, and safe color handling
- 📜 **Persistent history**: Searchable command history across sessions
- ⌨️ **Smart completion**: File system and command name completions
- ⚙️ **Scriptable**: Configuration via `.buxrc` with alias and environment variable support

## Getting Started

### Installation

#### From Source

1. Install Rust via [rustup](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/bux.git
   cd bux
   ```
3. Build:
   ```bash
   cargo build --release
   ```
4. Run:
   ```bash
   cargo run
   # or
   ./target/release/bux
   ```

### Configuration

Create a `~/.buxrc` file in your home directory:

```bash
# Aliases
alias ll="ls -l"
alias gs="git status"

# Environment variables
set EDITOR=vim
set PATH=/usr/local/bin:$PATH
```

## Usage

```bash
# Basic commands
bux> echo "Hello, bux!"
Hello, bux!

# Built-in commands
bux> cd /path/to/dir
bux> pwd
/path/to/dir
bux> alias ll="ls -l"
bux> history
bux> clear

# Exit
bux> exit
```

## Built-in Commands

- `cd [dir]` - Change directory
- `pwd` - Print working directory
- `alias [name="command"]` - List or set aliases
- `set VAR=value` - Set environment variables
- `history` - Show command history
- `clear` - Clear the terminal
- `source <file>` - Execute commands from a file
- `exit` - Exit the shell

## Architecture

```
┌────────────────────────────────────────────────────────┐
| bux (binary)                                           |
| ┌────────────┐  ┌──────────────┐  ┌───────────────┐   |
| | REPL/IO    |  | Parser/AST   |  | Executor      |   |
| | (rustyline)|  | (tokenizer)  |  | (builtins +   |   |
| |            |  |              |  |  external cmd) |   |
| └────────────┘  └──────────────┘  └───────────────┘   |
|      │                 │                  │            |
|  Prompt/CWD        AliasMgr            Plugin host      |
|  ThemeMgr          HistoryMgr          WASM/Lua host   |
└────────────────────────────────────────────────────────┘
```

## Development

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
bux/
├── src/
│   ├── main.rs          # Entry point and REPL loop
│   ├── command.rs       # Command execution
│   ├── builtins.rs      # Built-in commands
│   ├── parser.rs        # Command parsing
│   ├── prompt.rs        # Prompt rendering
│   ├── config.rs        # Configuration loading
│   ├── history.rs       # History management
│   ├── alias.rs         # Alias expansion
│   ├── plugin.rs        # Plugin system
│   ├── theme.rs         # Theme management
│   └── utils.rs         # Utility functions
├── Cargo.toml
└── README.md
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Advanced parser with pipes and redirection
- [ ] WASM plugin system
- [ ] Git integration in prompt
- [ ] Tab completion
- [ ] Command palette
- [ ] Theme system
- [ ] Windows installer (MSI)
- [ ] Homebrew formula
- [ ] Linux packages (deb/rpm)

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

## Acknowledgments

- Inspired by modern shells like Zsh, Fish, and PowerShell
- Built with [rustyline](https://github.com/kkawakami/rustyline) for the REPL experience
- Uses [colored](https://github.com/mackwic/colored) for terminal colors

---

**Made with 🪶 by the bux contributors**

