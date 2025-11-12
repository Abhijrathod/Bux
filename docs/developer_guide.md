# Developer Guide

## Building

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Build Commands

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific crate
cargo build -p bux-cli

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Structure

```
bux/
├── Cargo.toml          # Workspace definition
├── crates/             # All crates
│   ├── bux-core/       # Core runtime
│   ├── bux-commands/   # Built-in commands
│   ├── bux-cli/        # CLI interface
│   └── ...
├── tools/              # Build scripts
├── docs/               # Documentation
└── installers/         # Installer scripts
```

## Adding a New Command

1. Add command implementation to `crates/bux-commands/src/`
2. Register in `crates/bux-commands/src/lib.rs`
3. Add handler in `crates/bux-cli/src/builtins.rs`
4. Write tests in `crates/bux-tests/`

## Adding a New Crate

1. Create directory: `crates/my-crate/`
2. Add `Cargo.toml` with workspace inheritance
3. Add to workspace members in root `Cargo.toml`
4. Implement crate functionality

## Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p bux-core

# Run with output
cargo test -- --nocapture
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

