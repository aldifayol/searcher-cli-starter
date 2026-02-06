# Hybrid Crate Guide: Binary + Library

This document explains how the searcher project has been structured as a **hybrid crate** that can be used both as a command-line tool and as a library dependency.

## What is a Hybrid Crate?

A hybrid crate is a Rust project that provides both:
1. **Binary crate** - An executable program (CLI tool)
2. **Library crate** - Reusable code that other projects can import

## Project Structure

```
searcher-cli-starter/
├── src/
│   ├── lib.rs      # 📚 Library code (reusable functions)
│   └── main.rs     # 🔧 Binary code (CLI wrapper)
├── examples/
│   └── library_usage.rs  # Example of using as library
└── Cargo.toml      # Configuration
```

## How It Works

### src/lib.rs - The Library

Contains the core search logic that can be used by other Rust projects:

```rust
// Public API (marked with `pub`)
pub struct SearchMatch { ... }
pub enum Matcher { ... }
pub fn search_lines<R: Read>(...) -> Result<Vec<SearchMatch>>
```

**Key points:**
- All reusable types and functions are marked `pub`
- Contains comprehensive documentation with examples
- Includes all unit tests
- Can be imported by other projects

### src/main.rs - The Binary

A thin CLI wrapper that uses the library:

```rust
use searcher_cli_starter::{search_lines, Matcher};  // Import from lib

fn main() -> Result<()> {
    // Parse CLI arguments
    // Call library functions
    // Display results
}
```

**Key points:**
- Minimal code - just CLI interface
- Uses the library's public API
- Handles user interaction and output formatting

## Using as a CLI Tool

### Installation

```bash
cargo install searcher-cli-starter
```

### Usage

```bash
searcher "pattern" file.txt
searcher -i -n "error" logfile.txt
```

## Using as a Library

### Add Dependency

```toml
# In your project's Cargo.toml
[dependencies]
searcher-cli-starter = "0.2.0"
```

### Import and Use

```rust
use searcher_cli_starter::{Matcher, search_lines};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.txt")?;
    let matcher = Matcher::new("error", true, false)?;
    let results = search_lines(file, &matcher)?;

    for result in results {
        println!("Line {}: {}", result.line_number, result.content);
    }

    Ok(())
}
```

### Available API

#### SearchMatch

```rust
pub struct SearchMatch {
    pub line_number: usize,  // 1-based line number
    pub content: String,      // Line content
}
```

#### Matcher

```rust
pub enum Matcher {
    Literal { pattern: String, ignore_case: bool },
    Regex { regex: Regex },
}

impl Matcher {
    pub fn new(pattern: &str, ignore_case: bool, use_regex: bool) -> Result<Self>
    pub fn is_match(&self, line: &str) -> bool
}
```

#### search_lines

```rust
pub fn search_lines<R: Read>(
    reader: R,
    matcher: &Matcher
) -> Result<Vec<SearchMatch>>
```

## Examples

### Example 1: Basic Search

```rust
use searcher_cli_starter::{Matcher, search_lines};
use std::io::Cursor;

let text = "hello world\nrust is great\nhello rust";
let cursor = Cursor::new(text);

let matcher = Matcher::new("hello", false, false)?;
let results = search_lines(cursor, &matcher)?;

println!("Found {} matches", results.len());
```

### Example 2: Case-Insensitive

```rust
let matcher = Matcher::new("rust", true, false)?;  // ignore_case = true
let results = search_lines(cursor, &matcher)?;
// Matches: "Rust", "RUST", "rust"
```

### Example 3: Regex

```rust
let matcher = Matcher::new("^error:", false, true)?;  // use_regex = true
let results = search_lines(cursor, &matcher)?;
// Matches lines starting with "error:"
```

### Example 4: File Search

```rust
use std::fs::File;

let file = File::open("logfile.txt")?;
let matcher = Matcher::new("error", true, false)?;
let results = search_lines(file, &matcher)?;
```

## Running Examples

The project includes a complete example:

```bash
cargo run --example library_usage
```

This demonstrates:
- In-memory string search
- Case-insensitive search
- Regex patterns
- File search
- Result processing

## Benefits of Hybrid Crates

### For CLI Users
- Install and use as a command-line tool
- No need to know Rust

### For Developers
- Import as a library dependency
- Integrate into larger projects
- Customize behavior
- Build custom tools

### For Project
- Code reuse between binary and library
- Better testing (unit tests in lib, integration tests for CLI)
- Clear separation of concerns
- More flexible usage

## Documentation

### Generate Library Docs

```bash
cargo doc --no-deps --open
```

This generates comprehensive API documentation from the doc comments in `lib.rs`.

### View CLI Help

```bash
cargo run -- --help
```

## Testing

The project has comprehensive tests:

```bash
# Run all tests (unit + integration + doctests)
cargo test

# Run only library unit tests
cargo test --lib

# Run only integration tests (CLI)
cargo test --test integration_tests

# Run doctests (examples in documentation)
cargo test --doc
```

**Test Coverage:**
- **20 unit tests** in `lib.rs` - Test core functionality
- **21 integration tests** in `tests/` - Test CLI behavior
- **12 doctests** - Test examples in documentation

## Publishing to crates.io

To publish this hybrid crate:

```bash
# Login to crates.io
cargo login

# Dry run (check for issues)
cargo publish --dry-run

# Publish for real
cargo publish
```

Once published, users can:

```bash
# Install the CLI tool
cargo install searcher-cli-starter

# Add as library dependency
# (Just add to Cargo.toml as shown above)
```

## Real-World Examples

Many popular Rust projects are hybrid crates:

- **ripgrep** - CLI tool + `grep-searcher` library
- **cargo** - Build tool + `cargo` library
- **rustfmt** - Formatter tool + `rustfmt` library
- **clippy** - Linter tool + `clippy_lints` library

## Key Takeaways

1. **Library in lib.rs** - Core logic with `pub` items
2. **Binary in main.rs** - Thin CLI wrapper
3. **Import from library** - `use crate_name::*` in main.rs
4. **Two use cases** - Install as tool OR add as dependency
5. **Better architecture** - Clear separation of concerns
6. **More testable** - Unit tests in lib, integration tests for CLI

## Further Reading

- [The Cargo Book - Hybrid Crates](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Documentation Guidelines](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html)

---

**Summary**: This project can now be used both as a standalone CLI tool (`searcher`) and as a library that other Rust projects can depend on (`searcher-cli-starter` crate). This provides maximum flexibility and code reuse!
