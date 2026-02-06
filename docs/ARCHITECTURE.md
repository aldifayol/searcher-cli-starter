# Searcher CLI - Architecture Documentation

This document describes the internal architecture, design decisions, and implementation details of the Searcher CLI tool.

## Table of Contents

- [Overview](#overview)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Design Decisions](#design-decisions)
- [Extension Points](#extension-points)
- [Performance Considerations](#performance-considerations)
- [Future Architecture](#future-architecture)

## Overview

Searcher is designed as a simple, focused text search tool with three main responsibilities:

1. **Parse command-line arguments** - Convert user input into structured data
2. **Search file content** - Find lines matching a pattern
3. **Display results** - Format and output matching lines

The architecture prioritizes:
- **Simplicity** - Easy to understand and maintain
- **Performance** - Efficient for typical use cases
- **Extensibility** - Easy to add new features
- **Testability** - All components are testable

## Core Components

### 1. SearchMatch Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchMatch {
    line_number: usize,  // 1-based line number
    content: String,      // The actual line content
}
```

**Purpose**: Represents a single line that matched the search pattern.

**Design Rationale**:
- Combines line number with content for convenient handling
- Uses 1-based indexing to match editor conventions
- Implements `Clone` for flexibility in result handling
- Implements `PartialEq` and `Eq` for testing

**Trade-offs**:
- Small memory overhead (~8 bytes per match for line number)
- Enables future features like sorting, filtering, context display
- Keeps API clean and extensible

### 2. Matcher Enum

```rust
enum Matcher {
    Literal { pattern: String, ignore_case: bool },
    Regex { regex: Regex },
}
```

**Purpose**: Encapsulates pattern matching strategy (literal or regex).

**Design Rationale**:
- Enum allows efficient dispatch without dynamic dispatch overhead
- Separates matching strategy from I/O operations
- Enables compile-time regex validation
- Clean API with single `is_match()` method

**Alternative Approaches**:

1. **Trait-based design**:
   ```rust
   trait Matcher {
       fn is_match(&self, line: &str) -> bool;
   }
   ```
   - More extensible
   - Requires trait objects or generics
   - More complex for this use case

2. **Function pointer**:
   ```rust
   type MatchFn = fn(&str) -> bool;
   ```
   - Simplest approach
   - Loses context (pattern, flags)
   - Harder to test

**Trade-off**: Enum provides best balance of simplicity, performance, and extensibility for this application.

### 3. Matcher Implementation

```rust
impl Matcher {
    fn new(pattern: &str, ignore_case: bool, use_regex: bool) -> Result<Self>
    fn is_match(&self, line: &str) -> bool
}
```

**Key Methods**:

- **`new()`** - Constructs matcher with validation
  - Compiles regex patterns eagerly
  - Returns error for invalid regex
  - Converts case-insensitive to lowercase for literals
  - Adds `(?i)` flag for case-insensitive regex

- **`is_match()`** - Tests if a line matches the pattern
  - Literal matching uses `contains()`
  - Regex matching uses compiled `Regex::is_match()`

**Design Decisions**:

1. **Eager regex compilation**: Compile once in `new()` rather than on each match
   - Huge performance benefit
   - Clear error handling
   - Natural API

2. **Case-insensitive via `to_lowercase()`**: Simple and correct
   - Works for most use cases
   - Creates temporary strings
   - Alternative: Unicode normalization (more complex)

### 4. search_lines Function

```rust
fn search_lines<R: Read>(reader: R, matcher: &Matcher) -> Result<Vec<SearchMatch>>
```

**Purpose**: Core search algorithm that processes input line-by-line.

**Design Rationale**:
- Generic over `Read` trait for flexibility (files, strings, stdin)
- Returns `Vec<SearchMatch>` for convenient handling
- Uses `BufReader` for efficient I/O
- Tracks line numbers via `enumerate()`

**Performance Characteristics**:
- O(n) time complexity where n = number of lines
- O(m) space complexity where m = number of matches
- Streams data without loading entire file
- Efficient for files up to several GB

**Alternative Approaches**:

1. **Iterator-based**:
   ```rust
   fn search_lines<R: Read>(reader: R, matcher: &Matcher) -> impl Iterator<Item = SearchMatch>
   ```
   - More memory efficient (lazy)
   - More complex API
   - Overkill for this application

2. **Callback-based**:
   ```rust
   fn search_lines<R: Read, F: FnMut(SearchMatch)>(reader: R, matcher: &Matcher, callback: F)
   ```
   - No allocations for Vec
   - Awkward API
   - Doesn't compose well

### 5. Cli Struct

```rust
#[derive(Parser)]
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

**Purpose**: Parses and validates command-line arguments.

**Design Rationale**:
- Uses `clap` derive API for declarative parsing
- Short and long flag names match common tools (grep, ripgrep)
- Boolean flags default to false
- PathBuf provides OS-agnostic path handling

## Data Flow

### High-Level Flow

```
User Input
    ↓
Command-Line Parsing (Cli::parse)
    ↓
Matcher Creation (Matcher::new)
    ↓
File Opening (File::open)
    ↓
Search Execution (search_lines)
    ↓
Result Formatting (println!)
    ↓
Output Display
```

### Detailed Flow

1. **Argument Parsing**:
   ```rust
   let args = Cli::parse();
   // args.pattern = "rust"
   // args.ignore_case = true
   // args.path = PathBuf::from("file.txt")
   ```

2. **Matcher Construction**:
   ```rust
   let matcher = Matcher::new(&args.pattern, args.ignore_case, args.regex)?;
   // matcher = Matcher::Literal { pattern: "rust", ignore_case: true }
   ```

3. **File Opening**:
   ```rust
   let file = File::open(&args.path)?;
   // file = File handle to "file.txt"
   ```

4. **Search Execution**:
   ```rust
   let matches = search_lines(file, &matcher)?;
   // matches = vec![
   //     SearchMatch { line_number: 2, content: "Rust is great" },
   //     SearchMatch { line_number: 5, content: "I love Rust" },
   // ]
   ```

5. **Result Display**:
   ```rust
   for search_match in matches {
       if args.line_numbers {
           println!("{}:{}", search_match.line_number, search_match.content);
       } else {
           println!("{}", search_match.content);
       }
   }
   // Output:
   // 2:Rust is great
   // 5:I love Rust
   ```

## Design Decisions

### 1. Enum vs Trait for Matcher

**Decision**: Use enum with match expressions.

**Rationale**:
- Simpler implementation
- Better performance (no dynamic dispatch)
- Easier to test
- Sufficient for foreseeable features

**Trade-off**: Less extensible than trait-based design, but suitable for this application.

### 2. Eager vs Lazy Result Collection

**Decision**: Collect all matches into `Vec<SearchMatch>`.

**Rationale**:
- Simple API
- Easy to test
- Matches fit in memory for typical use cases
- Enables future features (sorting, deduplication)

**Trade-off**: Higher memory usage for large result sets, but acceptable.

### 3. 1-Based Line Numbering

**Decision**: Use 1-based indexing for line numbers.

**Rationale**:
- Matches editor conventions (vim, emacs, VS Code)
- Matches grep behavior
- More intuitive for users

**Implementation**: Enumerate from 0, then add 1 when creating `SearchMatch`.

### 4. Case-Insensitive Implementation

**Decision**: Use `to_lowercase()` for literal strings, `(?i)` flag for regex.

**Rationale**:
- Simple and correct
- No external dependencies
- Consistent between literal and regex
- Performance acceptable for line-by-line processing

**Alternative Considered**: Unicode normalization - more complex, overkill for this use case.

### 5. Error Handling Strategy

**Decision**: Use `anyhow::Result` with context.

**Rationale**:
- Simple error handling
- Good error messages
- Easy to add context
- Suitable for CLI applications

**Example**:
```rust
let file = File::open(&args.path)
    .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
```

### 6. Buffered I/O

**Decision**: Use `BufReader` for line-by-line reading.

**Rationale**:
- Significant performance improvement
- Standard Rust pattern
- Handles large files efficiently
- Minimal code complexity

## Extension Points

### Adding New Flags

To add a new boolean flag:

1. Add field to `Cli` struct:
   ```rust
   #[arg(short = 'v', long = "invert-match")]
   invert_match: bool,
   ```

2. Update logic in `main()` or `search_lines()`:
   ```rust
   if matcher.is_match(&content) != args.invert_match {
       matches.push(SearchMatch { ... });
   }
   ```

### Adding New Match Types

To add a new matching strategy:

1. Add variant to `Matcher` enum:
   ```rust
   enum Matcher {
       Literal { ... },
       Regex { ... },
       Fuzzy { pattern: String, threshold: f64 },
   }
   ```

2. Update `new()` and `is_match()`:
   ```rust
   Matcher::Fuzzy { pattern, threshold } => {
       fuzzy_match(line, pattern, threshold)
   }
   ```

### Adding Context Lines

To add context lines (lines before/after matches):

1. Change `SearchMatch` to include context:
   ```rust
   struct SearchMatch {
       line_number: usize,
       content: String,
       before: Vec<String>,
       after: Vec<String>,
   }
   ```

2. Update `search_lines()` to track context with ring buffer

### Adding Multiple File Support

To support multiple files:

1. Change `Cli` to accept multiple paths:
   ```rust
   paths: Vec<PathBuf>,
   ```

2. Add file name to `SearchMatch`:
   ```rust
   struct SearchMatch {
       file_name: String,
       line_number: usize,
       content: String,
   }
   ```

3. Loop over files in `main()`

## Performance Considerations

### Current Performance Characteristics

1. **Regex Compilation**: O(p) where p = pattern length
   - Done once in `Matcher::new()`
   - Huge win over per-line compilation

2. **Line Reading**: O(n) where n = number of lines
   - `BufReader` provides efficient buffering
   - No unnecessary allocations

3. **Matching**:
   - Literal: O(m) where m = line length
   - Regex: O(m) to O(m²) depending on pattern

4. **Memory Usage**:
   - Input: Streaming (one line at a time)
   - Output: O(k * m) where k = matches, m = avg line length

### Optimization Opportunities

1. **Parallel Processing**:
   - Use `rayon` to search multiple files in parallel
   - Chunk large files for parallel line processing

2. **Memory-Mapped I/O**:
   - Use `memmap2` for very large files
   - Reduces system call overhead

3. **String Interning**:
   - Intern common strings (file names, patterns)
   - Reduces memory for repeated strings

4. **SIMD Matching**:
   - Use SIMD for literal string matching
   - Requires careful implementation

### Performance Trade-offs

Current implementation prioritizes:
1. **Simplicity** over maximum performance
2. **Correctness** over speed
3. **Maintainability** over micro-optimizations

This is appropriate for a learning project and typical use cases.

## Future Architecture

### Planned Enhancements

1. **Plugin System**:
   - Allow custom matchers
   - Enable user-defined output formats

2. **Streaming Output**:
   - Print matches as found
   - Better for very large files

3. **Async I/O**:
   - Use tokio for async file I/O
   - Better for multiple files

4. **Configuration File**:
   - Support `.searcherrc` config
   - Allow setting default flags

### Architectural Evolution

As features are added, consider:

1. **Separate crate structure**:
   ```
   searcher/
   ├── searcher-core/    # Core library
   ├── searcher-cli/     # CLI binary
   └── searcher-plugins/ # Plugin system
   ```

2. **More abstraction**:
   - Trait for output formatters
   - Trait for input sources
   - Builder pattern for complex configuration

3. **Better error types**:
   - Custom error types instead of `anyhow`
   - More structured error reporting

## Testing Architecture

### Unit Tests

- Located in `src/main.rs` within `#[cfg(test)]` module
- Test individual functions in isolation
- Use `Cursor` for in-memory testing
- Cover success cases, failure cases, edge cases

### Integration Tests

- Located in `tests/integration_tests.rs`
- Test CLI behavior from user's perspective
- Use `assert_cmd` for command execution
- Test with real files and actual binary

### Test Coverage

Current coverage:
- **Core logic**: 100% (all branches tested)
- **CLI integration**: Comprehensive (21 tests)
- **Error handling**: Good (invalid regex, missing files)

## Summary

Searcher's architecture is designed to be:
- **Simple**: Easy to understand and modify
- **Efficient**: Good performance for typical use cases
- **Testable**: Comprehensive test coverage
- **Extensible**: Clear extension points

The current design serves the project's goals well and provides a solid foundation for future enhancements.

---

For questions or suggestions about the architecture, please open an issue on GitHub.
