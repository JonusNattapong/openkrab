# Contributing to krabkrab (Rust)

Thank you for your interest in contributing to krabkrab! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Porting Guidelines](#porting-guidelines)

## Code of Conduct

Be respectful and inclusive. We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/krabkrab.git
   cd krabkrab
   ```
3. Create a feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Setup

### Prerequisites

- Rust 1.75+ (use `rustup` for easy installation)
- Git

### Build

```bash
cargo build
```

### Run Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test module
cargo test connectors::discord --lib

# Run with verbose output
cargo test -- --nocapture
```

### Code Style

We use standard Rust conventions:

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Use meaningful variable and function names
- Add documentation comments (`///`) for public APIs

## Making Changes

1. **Check PORTING.md** — See what modules need work
2. **Write tests first** — TDD is encouraged
3. **Follow existing patterns** — Look at similar modules for structure
4. **Keep changes focused** — One feature/fix per PR

## Testing

### Test Structure

```
tests/
├── commands_test.rs      # CLI command tests
├── connectors_test.rs    # Connector integration tests
├── core_parity_test.rs   # Parity with TypeScript version
└── ...
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        // Arrange
        let input = "test";
        
        // Act
        let result = my_function(input);
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### Test Coverage

We aim for high test coverage. All new code should include tests.

## Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance tasks
- `port`: Porting from TypeScript

### Examples

```
feat(discord): add poll support

port(telegram): port message chunking from TS

fix(slack): handle rate limit errors correctly

docs(readme): update installation instructions
```

## Pull Request Process

1. **Create a branch** from `main`
2. **Make your changes** with clear commits
3. **Add/update tests** for your changes
4. **Run tests locally**: `cargo test --workspace`
5. **Run linter**: `cargo clippy -- -D warnings`
6. **Format code**: `cargo fmt`
7. **Push to your fork**
8. **Open a PR** against `main`

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated if applicable
- [ ] PORTING.md updated if applicable

## Porting Guidelines

When porting from the TypeScript version (`../openclaw`):

### Before Porting

1. Read the source module in TypeScript
2. Understand the module's purpose and dependencies
3. Check if there are existing tests to guide behavior
4. Look at PORTING.md for module mapping

### During Porting

1. **Keep the same behavior** — Don't change logic unless necessary
2. **Use idiomatic Rust** — Not a line-by-line translation
3. **Handle errors properly** — Use `Result` and `anyhow`
4. **Add documentation** — Document public APIs
5. **Write tests** — Port existing tests and add new ones

### After Porting

1. Update PORTING.md with the new module status
2. Add any new dependencies to Cargo.toml
3. Ensure all tests pass
4. Document any behavior differences in MIGRATION_NOTES.md

### Example Port

```rust
// TypeScript (openclaw/src/utils.ts)
export function normalizeText(text: string): string {
  return text.trim().toLowerCase();
}

// Rust (krabkrab/src/utils.rs)
/// Normalizes text by trimming and converting to lowercase.
pub fn normalize_text(text: &str) -> String {
    text.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("  HELLO  "), "hello");
    }
}
```

## Module Structure

Follow this structure for new modules:

```
src/
├── module_name/
│   ├── mod.rs       # Public interface
│   ├── types.rs     # Type definitions (optional)
│   └── tests.rs     # Tests (optional, or inline)
└── ...
```

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues before opening new ones
- Reference issues in commits and PRs

Thank you for contributing! 🦀
