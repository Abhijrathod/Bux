# Getting Started with bux

## Installation

### From Source

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
   cargo run --bin bux
   ```

## First Steps

1. Start the shell:
   ```bash
   bux
   ```

2. Try some commands:
   ```bash
   bux> pwd
   bux> cd ~
   bux> echo "Hello, bux!"
   ```

3. Configure your shell:
   Create `~/.buxrc`:
   ```bash
   alias ll="ls -l"
   alias gs="git status"
   set EDITOR=vim
   ```

## Next Steps

- Read the [Architecture Guide](architecture.md)
- Check out the [Developer Guide](developer_guide.md)
- Explore the [Module Specification](module_spec.md)

