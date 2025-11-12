# bux Architecture

## Overview

bux is structured as a Cargo workspace with multiple crates, similar to PowerShell's assembly structure.

## Crate Structure

### Core Crates

- **bux-core**: Core shell runtime, parser, executor, and value system
  - `parser.rs`: Command parsing and tokenization
  - `runtime.rs`: Shell runtime environment
  - `executor.rs`: Command execution
  - `pipeline.rs`: Pipeline processing
  - `value.rs`: Value type system

- **bux-commands**: Built-in commands
  - `filesystem.rs`: File system commands (cd, pwd, ls)
  - `network.rs`: Network commands
  - `process.rs`: Process management
  - `system.rs`: System information
  - `git.rs`: Git integration

- **bux-cli**: Command-line interface
  - `main.rs`: Entry point
  - `prompt.rs`: Prompt rendering
  - `history.rs`: Command history
  - `config.rs`: Configuration loading
  - `builtins.rs`: Built-in command handlers

### Platform Crates

- **bux-platform-unix**: Unix-specific code
- **bux-platform-win**: Windows-specific code

### Extension Crates

- **bux-sdk**: Plugin SDK
- **bux-security**: Security and permissions
- **bux-modules**: Native modules
- **bux-native**: Native system calls

## Data Flow

```
User Input → Parser → Runtime → Executor → Output
                ↓
            Pipeline → Commands → System
```

## Extension Points

1. **Plugins**: WASM-based plugins via bux-sdk
2. **Modules**: Native Rust modules
3. **Commands**: Built-in commands in bux-commands

