# Contributing to bux

Thank you for your interest in contributing to bux! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/yourusername/bux/issues)
2. If not, create a new issue with:
   - A clear, descriptive title
   - Steps to reproduce the bug
   - Expected vs. actual behavior
   - Your operating system and Rust version
   - Any relevant error messages or logs

### Suggesting Features

1. Check if the feature has already been suggested
2. Open an issue with:
   - A clear description of the feature
   - Use cases and examples
   - Any design considerations

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main` or `develop`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes following our coding standards
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test`
6. Format your code: `cargo fmt`
7. Run linter: `cargo clippy -- -D warnings`
8. Commit your changes with clear messages
9. Push to your fork and open a Pull Request

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code (we use default rustfmt settings)
- Run `cargo clippy` and fix all warnings
- Write documentation comments for public APIs using `///`

### Code Organization

- Keep functions focused and small
- Use meaningful variable and function names
- Add comments for complex logic
- Prefer `Result` types for error handling
- Use `anyhow` for error context

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Aim for good test coverage
- Test edge cases and error conditions

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(parser): add support for pipe operators

fix(prompt): resolve ANSI code spacing issue

docs(readme): update installation instructions
```

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/bux.git
   cd bux
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run the shell:
   ```bash
   cargo run
   ```

## Project Structure

- `src/main.rs` - Entry point and REPL loop
- `src/command.rs` - Command execution logic
- `src/builtins.rs` - Built-in shell commands
- `src/parser.rs` - Command parsing and tokenization
- `src/prompt.rs` - Prompt rendering
- `src/config.rs` - Configuration file loading
- `src/history.rs` - Command history management
- `src/alias.rs` - Alias expansion
- `src/plugin.rs` - Plugin system (WASM)
- `src/theme.rs` - Theme management
- `src/utils.rs` - Utility functions

## Review Process

1. All PRs require at least one maintainer review
2. CI must pass (formatting, linting, tests)
3. Maintainers may request changes
4. Once approved, a maintainer will merge the PR

## Questions?

Feel free to open an issue for questions or reach out to maintainers.

Thank you for contributing to bux! 🪶

