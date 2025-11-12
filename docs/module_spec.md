# Module Specification

## Overview

bux modules are similar to PowerShell modules - they provide commands, functions, and types.

## Module Types

### Native Modules (bux-modules)

Native Rust modules compiled into bux:
- System information
- Package management
- Platform-specific functionality

### WASM Modules (via bux-sdk)

Plugins compiled to WebAssembly:
- Sandboxed execution
- Cross-platform compatibility
- Secure third-party extensions

## Module Structure

A module should provide:

1. **Commands**: Executable commands
2. **Functions**: Reusable functions
3. **Types**: Custom value types
4. **Configuration**: Module-specific config

## Creating a Module

See the plugin template in `examples/plugin_template/` for WASM modules.

For native modules, add functionality to `crates/bux-modules/`.

