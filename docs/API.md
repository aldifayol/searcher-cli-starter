# Searcher CLI - Internal API Documentation

This document describes the internal API of Searcher CLI for developers who want to understand or extend the codebase.

## Table of Contents

- [Module Structure](#module-structure)
- [Public Types](#public-types)
- [Functions](#functions)
- [Error Handling](#error-handling)
- [Testing Utilities](#testing-utilities)

## Module Structure

```
src/
└── main.rs    # All code in single file (simple structure)
    ├── imports
    ├── SearchMatch struct
    ├── Matcher enum
    ├── Cli struct
    ├── search_lines function
    ├── main function
    └── tests module
```

## Public Types

### SearchMatch

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchMatch {
    line_number: usize,
    content: String,
}
```

**Purpose**: Represents a single line that matched the search pattern.

**Fields**:
- `line_number: usize` - The 1-based line number where the match was found
- `content: String` - The complete content of the matching line

**Traits Implemented**:
- `Debug` - For debugging output
- `Clone` - For creating copies
- `PartialEq, Eq` - For equality comparisons and testing

**Usage Example**:
```rust
let search_match = SearchMatch {
    line_number: 42,
    content: String::from("error in function"),
};

assert_eq!(search_match.line_number, 42);
assert_eq!(search_match.content, "error in function");
```

### Matcher

```rust
enum Matcher {
    Literal { pattern: String, ignore_case: bool },
    Regex { regex: Regex },
}
```

**Purpose**: Encapsulates pattern matching strategy (literal or regex).

**Variants**:

1. **`Literal { pattern: String, ignore_case: bool }`**
   - Matches literal string patterns
   - `pattern`: The search string (lowercase if `ignore_case` is true)
   - `ignore_case`: Whether matching is case-insensitive

2. **`Regex { regex: Regex }`**
   - Matches regular expression patterns
   - `regex`: Compiled regex from the `regex` crate

**Methods**:

#### `Matcher::new`

```rust
fn new(pattern: &str, ignore_case: bool, use_regex: bool) -> Result<Self>
```

Creates a new Matcher based on the provided parameters.

**Parameters**:
- `pattern: &str` - The search pattern (literal string or regex)
- `ignore_case: bool` - Whether to perform case-insensitive matching
- `use_regex: bool` - Whether to interpret pattern as a regular expression

**Returns**:
- `Result<Matcher>` - The constructed matcher or an error if regex is invalid

**Errors**:
- Returns `Err` if `use_regex` is true and the pattern is invalid regex

**Examples**:

```rust
// Literal matcher, case-sensitive
let matcher = Matcher::new("hello", false, false)?;

// Literal matcher, case-insensitive
let matcher = Matcher::new("hello", true, false)?;

// Regex matcher
let matcher = Matcher::new("h.*o", false, true)?;

// Regex matcher, case-insensitive
let matcher = Matcher::new("h.*o", true, true)?;

// Invalid regex returns error
let result = Matcher::new("[unclosed", false, true);
assert!(result.is_err());
```

**Implementation Details**:

For regex with case-insensitive:
```rust
// Pattern "hello" becomes "(?i)hello"
let regex_pattern = format!("(?i){}", pattern);
```

For literal with case-insensitive:
```rust
// Pattern is converted to lowercase for comparison
pattern.to_lowercase()
```

#### `Matcher::is_match`

```rust
fn is_match(&self, line: &str) -> bool
```

Tests if a line matches the pattern.

**Parameters**:
- `line: &str` - The line to test against the pattern

**Returns**:
- `bool` - `true` if the line matches, `false` otherwise

**Examples**:

```rust
let matcher = Matcher::new("rust", false, false)?;
assert!(matcher.is_match("rust is great"));
assert!(!matcher.is_match("python is great"));

let matcher = Matcher::new("rust", true, false)?;
assert!(matcher.is_match("Rust is great"));
assert!(matcher.is_match("RUST is great"));

let matcher = Matcher::new("r.st", false, true)?;
assert!(matcher.is_match("rust"));
assert!(matcher.is_match("rest"));
assert!(!matcher.is_match("rot"));
```

**Performance**:
- Literal matching: O(n) where n is line length
- Regex matching: O(n) to O(n²) depending on pattern complexity

### Cli

```rust
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: PathBuf,

    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,

    #[arg(short = 'r', long = "regex")]
    regex: bool,
}
```

**Purpose**: Parses and validates command-line arguments using clap.

**Fields**:
- `pattern: String` - The pattern to search for
- `path: PathBuf` - Path to the file to search
- `ignore_case: bool` - Whether to perform case-insensitive matching (default: false)
- `line_numbers: bool` - Whether to show line numbers (default: false)
- `regex: bool` - Whether to interpret pattern as regex (default: false)

**Usage**:
```rust
let args = Cli::parse();
// Automatically parses command-line arguments
```

**Generated Help**:
```
Usage: searcher [OPTIONS] <PATTERN> <PATH>

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

## Functions

### search_lines

```rust
fn search_lines<R: Read>(reader: R, matcher: &Matcher) -> Result<Vec<SearchMatch>>
```

**Purpose**: Searches through a reader line-by-line for lines matching the pattern.

**Type Parameters**:
- `R: Read` - Any type implementing the `Read` trait (File, Cursor, stdin, etc.)

**Parameters**:
- `reader: R` - The input source to search
- `matcher: &Matcher` - The matcher to use for pattern matching

**Returns**:
- `Result<Vec<SearchMatch>>` - Vector of all matching lines, or an error if I/O fails

**Errors**:
- Returns `Err` if reading from the input fails

**Algorithm**:
1. Wraps reader in `BufReader` for efficient line reading
2. Iterates through lines with `enumerate()` to track line numbers
3. For each line, checks if it matches using `matcher.is_match()`
4. Collects matching lines into `SearchMatch` structs
5. Returns vector of all matches

**Examples**:

```rust
use std::io::Cursor;

// Search in-memory string
let input = "hello world\nrust is great\nhello rust";
let cursor = Cursor::new(input);
let matcher = Matcher::new("hello", false, false)?;
let results = search_lines(cursor, &matcher)?;

assert_eq!(results.len(), 2);
assert_eq!(results[0].line_number, 1);
assert_eq!(results[0].content, "hello world");

// Search file
let file = File::open("sample.txt")?;
let matcher = Matcher::new("error", true, false)?;
let results = search_lines(file, &matcher)?;
```

**Performance**:
- Time complexity: O(n × m) where n = lines, m = average line length
- Space complexity: O(k × m) where k = number of matches
- Processes input line-by-line (streaming)
- Does not load entire file into memory

**Implementation Notes**:
```rust
for (line_number, line) in buf_reader.lines().enumerate() {
    let content = line?;  // Propagates I/O errors
    if matcher.is_match(&content) {
        matches.push(SearchMatch {
            line_number: line_number + 1,  // Convert to 1-based
            content,
        });
    }
}
```

### main

```rust
fn main() -> Result<()>
```

**Purpose**: Entry point for the CLI application.

**Flow**:
1. Parse command-line arguments with `Cli::parse()`
2. Open the specified file
3. Create a `Matcher` from pattern and flags
4. Search the file with `search_lines()`
5. Format and print results
6. Return success or propagate errors

**Error Handling**:
- File not found: Displays "Could not read file `path`"
- Invalid regex: Displays "Invalid regex pattern"
- I/O errors: Propagated with context

**Output Format**:

Without line numbers:
```
matching line 1
matching line 2
```

With line numbers:
```
5:matching line 1
12:matching line 2
```

## Error Handling

### Error Types

The application uses `anyhow::Result` for error handling:

```rust
type Result<T> = anyhow::Result<T>;
```

### Error Contexts

Errors are enriched with context using `.context()` and `.with_context()`:

```rust
let file = File::open(&args.path)
    .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
```

### Error Examples

#### File Not Found
```bash
$ searcher "pattern" nonexistent.txt
Error: Could not read file `nonexistent.txt`

Caused by:
    No such file or directory (os error 2)
```

#### Invalid Regex
```bash
$ searcher -r "[unclosed" file.txt
Error: Invalid regex pattern

Caused by:
    regex parse error:
        [unclosed
        ^
    error: unclosed character class
```

## Testing Utilities

### Test Module Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_name() {
        // Test implementation
    }
}
```

### Using Cursor for In-Memory Testing

```rust
use std::io::Cursor;

let input = "line 1\nline 2\nline 3";
let cursor = Cursor::new(input);

let matcher = Matcher::new("line 2", false, false)?;
let results = search_lines(cursor, &matcher)?;

assert_eq!(results.len(), 1);
assert_eq!(results[0].content, "line 2");
assert_eq!(results[0].line_number, 2);
```

### Common Test Patterns

#### Testing Literal Matching
```rust
#[test]
fn test_literal_match() {
    let input = "hello world\nrust is great";
    let cursor = Cursor::new(input);

    let matcher = Matcher::new("rust", false, false).unwrap();
    let results = search_lines(cursor, &matcher).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "rust is great");
}
```

#### Testing Case-Insensitive
```rust
#[test]
fn test_case_insensitive() {
    let input = "Hello\nHELLO\nhello";
    let cursor = Cursor::new(input);

    let matcher = Matcher::new("hello", true, false).unwrap();
    let results = search_lines(cursor, &matcher).unwrap();

    assert_eq!(results.len(), 3);
}
```

#### Testing Regex
```rust
#[test]
fn test_regex() {
    let input = "rust\nrest\nrat";
    let cursor = Cursor::new(input);

    let matcher = Matcher::new("r.st", false, true).unwrap();
    let results = search_lines(cursor, &matcher).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].content, "rust");
    assert_eq!(results[1].content, "rest");
}
```

#### Testing Error Cases
```rust
#[test]
fn test_invalid_regex() {
    let result = Matcher::new("[unclosed", false, true);
    assert!(result.is_err());
}
```

## Integration with External Code

### Using as a Library

Although designed as a CLI tool, the core functions can be used as a library:

```rust
use std::fs::File;

// Open file
let file = File::open("data.txt")?;

// Create matcher
let matcher = Matcher::new("error", true, false)?;

// Search
let matches = search_lines(file, &matcher)?;

// Process results
for search_match in matches {
    println!("Line {}: {}", search_match.line_number, search_match.content);
}
```

### Type Constraints

- `search_lines` is generic over `R: Read`, so it works with:
  - `File`
  - `Cursor<T>` where T implements `AsRef<[u8]>`
  - `std::io::stdin()`
  - Any custom type implementing `Read`

## Performance Characteristics

### Time Complexity

- `Matcher::new()`: O(p) for regex compilation, O(1) for literal
- `Matcher::is_match()`: O(m) for literal, O(m) to O(m²) for regex
- `search_lines()`: O(n × m) where n = lines, m = avg line length

### Space Complexity

- `Matcher`: O(p) where p = pattern length
- `SearchMatch`: O(m) where m = line length
- `search_lines()`: O(k × m) where k = number of matches

### Optimization Notes

- Regex is compiled once in `Matcher::new()`, not per-match
- `BufReader` provides buffering for efficient I/O
- Results are collected into Vec (not streaming) for simplicity

## Dependencies

### External Crates

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # CLI parsing
anyhow = "1.0"                                     # Error handling
regex = "1.10"                                     # Regular expressions
```

### Standard Library

- `std::fs::File` - File I/O
- `std::io::{BufRead, BufReader, Read}` - Buffered reading
- `std::path::PathBuf` - Path handling

## Summary

The Searcher CLI internal API consists of:

1. **SearchMatch** - Represents a matching line with line number
2. **Matcher** - Encapsulates matching strategy (literal or regex)
3. **search_lines()** - Core search algorithm
4. **Cli** - Command-line argument parser
5. **main()** - Application entry point

The design prioritizes simplicity, testability, and extensibility while maintaining good performance for typical use cases.

---

For questions about the internal API, please open an issue on GitHub.
