//! Searcher - A fast, flexible text search library with regex support.
//!
//! This crate provides text search functionality with support for:
//! - Case-insensitive matching
//! - Regular expression patterns
//! - Line number tracking
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use searcher_cli_starter::{Matcher, search_lines};
//! use std::io::Cursor;
//!
//! let input = "hello world\nrust is great\nhello rust";
//! let cursor = Cursor::new(input);
//!
//! let matcher = Matcher::new("hello", false, false).unwrap();
//! let results = search_lines(cursor, &matcher).unwrap();
//!
//! assert_eq!(results.len(), 2);
//! assert_eq!(results[0].line_number, 1);
//! assert_eq!(results[0].content, "hello world");
//! ```
//!
//! ## Case-Insensitive Search
//!
//! ```
//! use searcher_cli_starter::{Matcher, search_lines};
//! use std::io::Cursor;
//!
//! let input = "Rust\nRUST\nrust";
//! let cursor = Cursor::new(input);
//!
//! let matcher = Matcher::new("rust", true, false).unwrap();
//! let results = search_lines(cursor, &matcher).unwrap();
//!
//! assert_eq!(results.len(), 3);  // Matches all variants
//! ```
//!
//! ## Regular Expression Search
//!
//! ```
//! use searcher_cli_starter::{Matcher, search_lines};
//! use std::io::Cursor;
//!
//! let input = "rust\nrest\nrat";
//! let cursor = Cursor::new(input);
//!
//! let matcher = Matcher::new("r.st", false, true).unwrap();
//! let results = search_lines(cursor, &matcher).unwrap();
//!
//! assert_eq!(results.len(), 2);  // Matches "rust" and "rest"
//! ```
//!
//! ## Using with Files
//!
//! ```no_run
//! use searcher_cli_starter::{Matcher, search_lines};
//! use std::fs::File;
//!
//! let file = File::open("data.txt").unwrap();
//! let matcher = Matcher::new("error", true, false).unwrap();
//! let results = search_lines(file, &matcher).unwrap();
//!
//! for result in results {
//!     println!("Line {}: {}", result.line_number, result.content);
//! }
//! ```

use anyhow::{Context, Result};
use regex::Regex;
use std::io::{BufRead, BufReader, Read};

/// Represents a single line that matched the search pattern.
///
/// This struct captures both the line number (1-based) and the actual
/// content of the matching line. Line numbers are included even when
/// not displayed, allowing for efficient filtering and sorting.
///
/// # Examples
///
/// ```
/// use searcher_cli_starter::SearchMatch;
///
/// let search_match = SearchMatch {
///     line_number: 42,
///     content: String::from("error in function"),
/// };
///
/// assert_eq!(search_match.line_number, 42);
/// assert_eq!(search_match.content, "error in function");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    /// The line number where the match was found (1-based indexing)
    pub line_number: usize,
    /// The complete content of the matching line
    pub content: String,
}

/// Pattern matching strategy.
///
/// Supports both literal string matching and regular expression patterns.
/// The matcher is constructed once and then used repeatedly for efficient searching.
///
/// # Examples
///
/// ```
/// use searcher_cli_starter::Matcher;
///
/// // Create a literal matcher (case-sensitive)
/// let matcher = Matcher::new("hello", false, false).unwrap();
/// assert!(matcher.is_match("hello world"));
/// assert!(!matcher.is_match("Hello world"));
///
/// // Create a case-insensitive matcher
/// let matcher = Matcher::new("hello", true, false).unwrap();
/// assert!(matcher.is_match("Hello world"));
/// assert!(matcher.is_match("HELLO world"));
///
/// // Create a regex matcher
/// let matcher = Matcher::new("h.*o", false, true).unwrap();
/// assert!(matcher.is_match("hello"));
/// assert!(matcher.is_match("hero"));
/// ```
pub enum Matcher {
    /// Literal string matching with optional case-insensitive comparison
    Literal {
        /// The pattern to match (lowercase if ignore_case is true)
        pattern: String,
        /// Whether to perform case-insensitive matching
        ignore_case: bool,
    },
    /// Regular expression matching using the regex crate
    Regex {
        /// The compiled regular expression
        regex: Regex,
    },
}

impl Matcher {
    /// Creates a new Matcher based on the provided pattern and flags.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The search pattern (literal string or regex)
    /// * `ignore_case` - Whether to perform case-insensitive matching
    /// * `use_regex` - Whether to interpret the pattern as a regular expression
    ///
    /// # Returns
    ///
    /// Returns a Result containing the Matcher or an error if the regex pattern is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if `use_regex` is true and the pattern is not valid regex syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use searcher_cli_starter::Matcher;
    ///
    /// // Literal matcher
    /// let matcher = Matcher::new("hello", false, false).unwrap();
    ///
    /// // Case-insensitive literal matcher
    /// let matcher = Matcher::new("hello", true, false).unwrap();
    ///
    /// // Regex matcher
    /// let matcher = Matcher::new("h.*o", false, true).unwrap();
    ///
    /// // Invalid regex returns error
    /// let result = Matcher::new("[unclosed", false, true);
    /// assert!(result.is_err());
    /// ```
    pub fn new(pattern: &str, ignore_case: bool, use_regex: bool) -> Result<Self> {
        if use_regex {
            let regex_pattern = if ignore_case {
                format!("(?i){}", pattern)
            } else {
                pattern.to_string()
            };
            let regex = Regex::new(&regex_pattern).context("Invalid regex pattern")?;
            Ok(Matcher::Regex { regex })
        } else {
            Ok(Matcher::Literal {
                pattern: if ignore_case {
                    pattern.to_lowercase()
                } else {
                    pattern.to_string()
                },
                ignore_case,
            })
        }
    }

    /// Checks if the given line matches the pattern.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to test against the pattern
    ///
    /// # Returns
    ///
    /// Returns true if the line matches the pattern, false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use searcher_cli_starter::Matcher;
    ///
    /// let matcher = Matcher::new("rust", false, false).unwrap();
    /// assert!(matcher.is_match("rust is great"));
    /// assert!(!matcher.is_match("python is great"));
    ///
    /// let matcher = Matcher::new("rust", true, false).unwrap();
    /// assert!(matcher.is_match("Rust is great"));
    /// assert!(matcher.is_match("RUST is great"));
    ///
    /// let matcher = Matcher::new("r.st", false, true).unwrap();
    /// assert!(matcher.is_match("rust"));
    /// assert!(matcher.is_match("rest"));
    /// assert!(!matcher.is_match("rot"));
    /// ```
    pub fn is_match(&self, line: &str) -> bool {
        match self {
            Matcher::Literal {
                pattern,
                ignore_case,
            } => {
                if *ignore_case {
                    line.to_lowercase().contains(pattern)
                } else {
                    line.contains(pattern)
                }
            }
            Matcher::Regex { regex } => regex.is_match(line),
        }
    }
}

/// Searches through a reader line-by-line for lines matching the pattern.
///
/// This function processes input line-by-line using buffered I/O for efficiency.
/// It works with any type implementing the `Read` trait, including files, strings,
/// and standard input.
///
/// # Arguments
///
/// * `reader` - Any type implementing Read (files, strings, stdin, etc.)
/// * `matcher` - The Matcher to use for pattern matching
///
/// # Returns
///
/// Returns a Result containing a Vec of SearchMatch structs for all matching lines,
/// or an error if reading fails.
///
/// # Errors
///
/// Returns an error if:
/// - Reading from the input source fails
/// - A line contains invalid UTF-8
///
/// # Performance
///
/// - Time complexity: O(n × m) where n = number of lines, m = average line length
/// - Space complexity: O(k × m) where k = number of matches
/// - Streams input line-by-line without loading entire file into memory
///
/// # Examples
///
/// ## Searching in-memory strings
///
/// ```
/// use searcher_cli_starter::{Matcher, search_lines};
/// use std::io::Cursor;
///
/// let input = "hello world\nrust is great\nhello rust";
/// let cursor = Cursor::new(input);
///
/// let matcher = Matcher::new("hello", false, false).unwrap();
/// let results = search_lines(cursor, &matcher).unwrap();
///
/// assert_eq!(results.len(), 2);
/// assert_eq!(results[0].line_number, 1);
/// assert_eq!(results[0].content, "hello world");
/// assert_eq!(results[1].line_number, 3);
/// assert_eq!(results[1].content, "hello rust");
/// ```
///
/// ## Searching files
///
/// ```no_run
/// use searcher_cli_starter::{Matcher, search_lines};
/// use std::fs::File;
///
/// let file = File::open("data.txt").unwrap();
/// let matcher = Matcher::new("error", true, false).unwrap();
/// let results = search_lines(file, &matcher).unwrap();
///
/// for result in results {
///     println!("Line {}: {}", result.line_number, result.content);
/// }
/// ```
///
/// ## Case-insensitive search
///
/// ```
/// use searcher_cli_starter::{Matcher, search_lines};
/// use std::io::Cursor;
///
/// let input = "Rust\nRUST\nrust";
/// let cursor = Cursor::new(input);
///
/// let matcher = Matcher::new("rust", true, false).unwrap();
/// let results = search_lines(cursor, &matcher).unwrap();
///
/// assert_eq!(results.len(), 3);
/// ```
///
/// ## Regex search
///
/// ```
/// use searcher_cli_starter::{Matcher, search_lines};
/// use std::io::Cursor;
///
/// let input = "rust\nrest\nrat";
/// let cursor = Cursor::new(input);
///
/// let matcher = Matcher::new("r.st", false, true).unwrap();
/// let results = search_lines(cursor, &matcher).unwrap();
///
/// assert_eq!(results.len(), 2);
/// ```
pub fn search_lines<R: Read>(reader: R, matcher: &Matcher) -> Result<Vec<SearchMatch>> {
    let buf_reader = BufReader::new(reader);
    let mut matches = Vec::new();

    for (line_number, line) in buf_reader.lines().enumerate() {
        let content = line?;
        if matcher.is_match(&content) {
            matches.push(SearchMatch {
                line_number: line_number + 1, // 1-based indexing
                content,
            });
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_search_finds_matching_lines() {
        let input = "hello world\nrust is great\nhello rust\nfarewell";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("hello", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "hello world");
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].content, "hello rust");
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_search_no_matches() {
        let input = "foo\nbar\nbaz";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("nonexistent", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_case_sensitive() {
        let input = "Hello World\nhello world";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("hello", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "hello world");
        assert_eq!(results[0].line_number, 2);
    }

    #[test]
    fn test_search_empty_input() {
        let input = "";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("anything", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_partial_match() {
        let input = "testing\ntest\ncontest";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("test", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 2);
        assert_eq!(results[2].line_number, 3);
    }

    // Case-insensitive tests
    #[test]
    fn test_case_insensitive_lowercase_pattern() {
        let input = "Hello World\nRUST\nrust programming";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("rust", true, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "RUST");
        assert_eq!(results[1].content, "rust programming");
    }

    #[test]
    fn test_case_insensitive_uppercase_pattern() {
        let input = "rust is cool\nRust programming\nRUST";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("RUST", true, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_case_insensitive_mixed_case() {
        let input = "RuSt\nrust\nRUST\nrust_lang";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("RuSt", true, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 4);
    }

    // Line number tests
    #[test]
    fn test_line_numbers_first_line() {
        let input = "match this\nno match\nno match";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("match this", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 1);
    }

    #[test]
    fn test_line_numbers_multiple_matches() {
        let input = "line 1\nmatch\nline 3\nmatch\nline 5";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("match", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 2);
        assert_eq!(results[1].line_number, 4);
    }

    #[test]
    fn test_line_numbers_correct_ordering() {
        let input = "a\nb\nc\nmatch\ne\nmatch\ng";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("match", false, false).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results[0].line_number, 4);
        assert_eq!(results[1].line_number, 6);
    }

    // Regex tests
    #[test]
    fn test_regex_dot_wildcard() {
        let input = "rust\nrest\nroast\nrat";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("r.st", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "rust");
        assert_eq!(results[1].content, "rest");
    }

    #[test]
    fn test_regex_start_anchor() {
        let input = "rust is great\nI love rust\nrust";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("^rust", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "rust is great");
        assert_eq!(results[1].content, "rust");
    }

    #[test]
    fn test_regex_end_anchor() {
        let input = "rust\nlove rust\nrust is";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("rust$", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "rust");
        assert_eq!(results[1].content, "love rust");
    }

    #[test]
    fn test_regex_character_class() {
        let input = "rust\nRust\nrest\ntest";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("[Rr]ust", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "rust");
        assert_eq!(results[1].content, "Rust");
    }

    #[test]
    fn test_regex_quantifiers() {
        let input = "bt\nbet\nbeet\nbeeet";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("be+t", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 3);
        assert!(!results.iter().any(|m| m.content == "bt"));
    }

    #[test]
    fn test_regex_word_boundary() {
        let input = "rust\nrust_lang\ntrustworthy";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new(r"\brust\b", false, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "rust");
    }

    #[test]
    fn test_regex_case_insensitive_combined() {
        let input = "Rust\nRUST\nrust";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("rust", true, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_invalid_regex_returns_error() {
        let result = Matcher::new("[unclosed", false, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_features_combined() {
        let input = "RUST is great\nrust programming\nRust language";
        let cursor = Cursor::new(input);

        let matcher = Matcher::new("R.*T", true, true).unwrap();
        let results = search_lines(cursor, &matcher).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 2);
        assert_eq!(results[2].line_number, 3);
    }
}
