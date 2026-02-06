# Contributing to Searcher CLI

Thank you for your interest in contributing to Searcher CLI! This document provides guidelines and information for contributors.

## Code of Conduct

This project follows a simple code of conduct:

- Be respectful and considerate
- Be collaborative and constructive
- Focus on what is best for the community
- Show empathy towards other community members

## How to Contribute

There are many ways to contribute to this project:

1. **Report bugs** - Submit detailed bug reports via GitHub Issues
2. **Suggest features** - Propose new features or enhancements
3. **Improve documentation** - Fix typos, clarify explanations, add examples
4. **Write code** - Implement new features or fix bugs
5. **Review pull requests** - Provide feedback on others' contributions
6. **Answer questions** - Help users in Discussions

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

### Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/searcher-cli-starter.git
   cd searcher-cli-starter
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/yourusername/searcher-cli-starter.git
   ```

4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

5. **Build and test**:
   ```bash
   cargo build
   cargo test
   ```

## Development Workflow

### Before You Start

1. **Check existing issues** - See if someone is already working on it
2. **Open an issue** - Discuss your idea before spending time on it
3. **Get feedback** - Make sure your approach aligns with project goals

### Making Changes

1. **Write your code**
   - Follow the code style guidelines below
   - Keep changes focused and atomic
   - Write clear, self-documenting code

2. **Add tests**
   - Every new feature needs tests
   - Every bug fix should have a test that would have caught it
   - Aim for high test coverage

3. **Update documentation**
   - Update README.md if you change user-facing behavior
   - Update inline doc comments for public APIs
   - Add examples for new features

4. **Run the full test suite**:
   ```bash
   cargo test
   ```

5. **Format your code**:
   ```bash
   cargo fmt
   ```

6. **Lint your code**:
   ```bash
   cargo clippy -- -D warnings
   ```

### Submitting Changes

1. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Add feature: brief description"
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Open a Pull Request** on GitHub
   - Use the PR template
   - Link any related issues
   - Provide a clear description of changes

## Code Style Guidelines

### Rust Code Style

We follow standard Rust conventions:

- **Use `rustfmt`** - Run `cargo fmt` before committing
- **Follow `clippy`** - Fix all warnings from `cargo clippy`
- **Use idiomatic Rust** - Follow patterns from the Rust book
- **Prefer explicit types** - When it improves readability
- **Keep functions small** - Break complex logic into smaller functions

### Naming Conventions

- `snake_case` for variables and functions
- `PascalCase` for types and structs
- `SCREAMING_SNAKE_CASE` for constants
- Descriptive names that explain purpose

### Documentation Style

```rust
/// Brief one-line summary.
///
/// More detailed explanation if needed. Can span multiple
/// paragraphs.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value.
///
/// # Examples
///
/// ```
/// let result = function(arg1, arg2);
/// assert_eq!(result, expected);
/// ```
fn function(arg1: Type1, arg2: Type2) -> ReturnType {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `anyhow::Result` for application errors
- Provide context with `.context()` or `.with_context()`
- Write helpful error messages

### Testing

```rust
#[test]
fn test_descriptive_name() {
    // Arrange
    let input = setup_test_data();

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected);
}
```

## Testing Guidelines

### Unit Tests

- Place unit tests in the same file as the code they test
- Use the `#[cfg(test)]` module pattern
- Test both success and failure cases
- Test edge cases (empty input, boundary conditions, etc.)

### Integration Tests

- Place integration tests in `tests/` directory
- Test the CLI interface from a user's perspective
- Use `assert_cmd` for command-line testing
- Test with real files when possible

### Writing Good Tests

- **Clear test names** - `test_feature_when_condition_then_result`
- **One assertion per test** - Each test should verify one thing
- **Use descriptive assertions** - Make failures easy to diagnose
- **Avoid test interdependencies** - Tests should run independently
- **Test behavior, not implementation** - Tests should survive refactoring

## Commit Message Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Formatting, missing semicolons, etc.
- `refactor` - Code restructuring without behavior change
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

### Examples

```
feat: add case-insensitive search flag

Implement -i/--ignore-case flag to enable case-insensitive
pattern matching for both literal and regex patterns.

Closes #42
```

```
fix: correct line number indexing in search results

Line numbers were 0-based but should be 1-based to match
standard editor conventions.
```

```
docs: add regex syntax examples to README

Include common regex patterns like email matching and
word boundaries to help users learn the feature.
```

## Pull Request Process

### PR Checklist

Before submitting your PR, ensure:

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for notable changes)
- [ ] Commit messages follow guidelines
- [ ] PR description explains the changes

### PR Template

When opening a PR, include:

1. **Summary** - Brief description of changes
2. **Motivation** - Why is this change needed?
3. **Changes** - What specifically changed?
4. **Testing** - How was this tested?
5. **Related Issues** - Link to any related issues

### Review Process

1. A maintainer will review your PR
2. Address any feedback or requested changes
3. Once approved, a maintainer will merge your PR
4. Your contribution will be included in the next release

## Project Architecture

### Core Components

- **`SearchMatch`** - Represents a matching line with line number
- **`Matcher`** - Enum for literal vs regex matching strategies
- **`search_lines`** - Core search function
- **`Cli`** - Command-line argument parser

### Design Principles

- **Simple and focused** - Do one thing well
- **Composable** - Features work together naturally
- **Performant** - Efficient for typical use cases
- **Well-tested** - Comprehensive test coverage
- **Well-documented** - Clear docs and examples

### Adding New Features

When adding features, consider:

1. **Does it fit the project scope?** - Text search tool
2. **Is it composable?** - Works with existing flags
3. **Is it backward compatible?** - Don't break existing usage
4. **Is it well-tested?** - Add comprehensive tests
5. **Is it documented?** - Update README and docs

## Getting Help

- **Questions** - Open a Discussion on GitHub
- **Bugs** - Open an Issue with reproduction steps
- **Chat** - Join our community (link TBD)

## Recognition

Contributors are recognized in:

- Git commit history
- Release notes in CHANGELOG.md
- GitHub contributors page

Thank you for contributing to Searcher CLI!

---

## Quick Reference

### Common Commands

```bash
# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy

# Run
cargo run -- "pattern" file.txt

# Build release
cargo build --release
```

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation
- `refactor/description` - Refactoring

### Useful Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clap Documentation](https://docs.rs/clap/)
- [Regex Documentation](https://docs.rs/regex/)
