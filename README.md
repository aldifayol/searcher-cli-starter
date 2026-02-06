# Searcher CLI

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/searcher-cli-starter)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.2.0-orange.svg)](Cargo.toml)

A fast, flexible command-line text search tool with regex support, built in Rust.

## Overview

Searcher is a lightweight command-line utility for searching text files. It provides powerful pattern matching capabilities with support for case-insensitive search, line numbers, and regular expressions. Think of it as a simplified, learning-focused alternative to tools like `grep`.

## Features

- **Fast text search** - Efficient line-by-line searching using buffered I/O
- **Case-insensitive matching** - Search without worrying about capitalization
- **Line numbers** - Display line numbers alongside matching lines
- **Regular expressions** - Powerful pattern matching with full regex support
- **Clear error messages** - Helpful feedback when things go wrong
- **Composable flags** - Combine multiple features for complex searches
- **Zero configuration** - Works out of the box

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/searcher-cli-starter.git
cd searcher-cli-starter

# Build the release binary
cargo build --release

# The binary will be at target/release/searcher
```

### Future: From crates.io

```bash
# Coming soon
cargo install searcher-cli-starter
```

## Quick Start

### As a CLI Tool

```bash
# Basic search
searcher "pattern" file.txt

# Case-insensitive search
searcher -i "rust" file.txt

# Show line numbers
searcher -n "error" file.txt

# Use regular expressions
searcher -r "^\[ERROR\]" logfile.txt

# Combine multiple flags
searcher -i -n -r "warning|error" file.txt
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
searcher-cli-starter = "0.2.0"
```

Use in your code:

```rust
use searcher_cli_starter::{Matcher, search_lines};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a file
    let file = File::open("data.txt")?;

    // Create a matcher (pattern, case_insensitive, use_regex)
    let matcher = Matcher::new("error", true, false)?;

    // Search the file
    let results = search_lines(file, &matcher)?;

    // Process results
    for result in results {
        println!("Line {}: {}", result.line_number, result.content);
    }

    Ok(())
}
```

See `examples/library_usage.rs` for more detailed examples.

## Usage

```
searcher [OPTIONS] <PATTERN> <PATH>

Arguments:
  <PATTERN>  The pattern to look for
  <PATH>     The path to the file to read

Options:
  -i, --ignore-case     Perform case-insensitive matching
  -n, --line-numbers    Show line numbers with output lines
  -r, --regex           Interpret pattern as a regular expression
  -h, --help            Print help
  -V, --version         Print version
```

## Examples

### Basic Search

Search for the word "Rust" in a file:

```bash
searcher "Rust" sample.txt
```

Output:
```
Rust is a systems programming language
Hello world from Rust
Rust makes systems programming accessible
```

### Case-Insensitive Search

Find all occurrences regardless of case:

```bash
searcher -i "rust" sample.txt
```

This will match "Rust", "RUST", "rust", etc.

### Line Numbers

Display line numbers for context:

```bash
searcher -n "error" logfile.txt
```

Output:
```
15:Connection error occurred
42:Timeout error in handler
89:Database error: connection refused
```

### Regular Expressions

Use regex patterns for complex matching:

```bash
# Match lines starting with "[ERROR]"
searcher -r "^\[ERROR\]" logfile.txt

# Match email addresses
searcher -r "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" contacts.txt

# Match words ending with "ing"
searcher -r "\w+ing\b" document.txt

# Match IP addresses
searcher -r "\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b" network.log
```

### Combining Flags

All flags can be combined for powerful searches:

```bash
# Case-insensitive regex with line numbers
searcher -i -n -r "warning|error|critical" system.log
```

Output:
```
12:WARNING: Low memory available
45:Error in authentication module
78:CRITICAL: Database connection lost
```

## Command-Line Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-i` | `--ignore-case` | Perform case-insensitive matching. The pattern will match regardless of letter case. |
| `-n` | `--line-numbers` | Show line numbers with output lines. Format is `N:content` where N is 1-based. |
| `-r` | `--regex` | Interpret the pattern as a regular expression. Enables powerful pattern matching. |
| `-h` | `--help` | Print help information including all options and usage. |
| `-V` | `--version` | Print the version number of searcher. |

## Regular Expression Syntax

When using the `-r` flag, searcher supports the full regex syntax provided by Rust's `regex` crate:

- `.` - Match any character except newline
- `^` - Match start of line
- `$` - Match end of line
- `*` - Match 0 or more times
- `+` - Match 1 or more times
- `?` - Match 0 or 1 time
- `{n}` - Match exactly n times
- `{n,}` - Match n or more times
- `{n,m}` - Match between n and m times
- `[abc]` - Match any character in the set
- `[^abc]` - Match any character not in the set
- `\d` - Match any digit
- `\w` - Match any word character
- `\s` - Match any whitespace
- `\b` - Match word boundary
- `(pattern)` - Capture group
- `|` - Alternation (OR)

For more details, see the [regex crate documentation](https://docs.rs/regex/).

## Building from Source

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/searcher-cli-starter.git
cd searcher-cli-starter

# Run tests to verify everything works
cargo test

# Build the debug version (faster compilation)
cargo build

# Or build the optimized release version
cargo build --release

# Run the binary
./target/release/searcher --help
```

### Development Build

For development, use the debug build:

```bash
cargo build
cargo run -- "pattern" file.txt
```

## Running Tests

Searcher has comprehensive test coverage including unit tests and integration tests:

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run a specific test
cargo test test_case_insensitive
```

The test suite includes:
- **20 unit tests** covering core functionality
- **21 integration tests** covering CLI behavior
- Tests for all feature combinations
- Error handling tests
- Backward compatibility tests

## Project Structure

```
searcher-cli-starter/
├── src/
│   └── main.rs           # Main source code
├── tests/
│   ├── fixtures/
│   │   └── sample.txt    # Test data file
│   └── integration_tests.rs  # Integration tests
├── docs/
│   ├── ARCHITECTURE.md   # Technical architecture
│   ├── EXAMPLES.md       # Extended examples
│   └── API.md            # Internal API docs
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── Cargo.toml            # Project manifest
├── README.md             # This file
├── CHANGELOG.md          # Version history
├── CONTRIBUTING.md       # Contributor guidelines
└── LICENSE               # Project license
```

## Performance

Searcher is designed for efficiency:

- **Buffered I/O**: Uses `BufReader` for efficient line-by-line reading
- **Regex compilation**: Compiles regex patterns once before searching
- **Minimal allocations**: Efficient memory usage during search
- **Streaming**: Processes files line-by-line without loading into memory

For typical use cases (searching files up to several MB), searcher performs comparably to standard Unix tools.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run tests (`cargo test`)
6. Format code (`cargo fmt`)
7. Lint code (`cargo clippy`)
8. Commit changes (`git commit -m 'Add amazing feature'`)
9. Push to branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

- Built with [clap](https://docs.rs/clap/) for CLI argument parsing
- Uses [regex](https://docs.rs/regex/) for pattern matching
- Error handling via [anyhow](https://docs.rs/anyhow/)
- Inspired by classic Unix tools like `grep` and modern alternatives like `ripgrep`

## Support

- Report bugs via [GitHub Issues](https://github.com/yourusername/searcher-cli-starter/issues)
- Ask questions in [Discussions](https://github.com/yourusername/searcher-cli-starter/discussions)
- Read the full documentation in the [docs/](docs/) directory

## Roadmap

Potential future enhancements:

- Color output with ANSI codes
- Context lines (`-A`, `-B`, `-C` flags)
- Multiple file support
- Recursive directory search
- Invert match (`-v` flag)
- Count-only mode (`-c` flag)
- Performance benchmarks
- Publication to crates.io

## Version History

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

## Authors

- Aldi - Initial work

---

Made with ❤️ in Rust
