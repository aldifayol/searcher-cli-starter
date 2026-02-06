use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

/// Represents a single line that matched the search pattern.
///
/// This struct captures both the line number (1-based) and the actual
/// content of the matching line. Line numbers are included even when
/// not displayed, allowing for efficient filtering and sorting.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchMatch {
    /// The line number where the match was found (1-based indexing)
    line_number: usize,
    /// The complete content of the matching line
    content: String,
}

/// Pattern matching strategy.
///
/// Supports both literal string matching and regular expression patterns.
/// The matcher is constructed once and then used repeatedly for efficient searching.
enum Matcher {
    /// Literal string matching with optional case-insensitive comparison
    Literal { pattern: String, ignore_case: bool },
    /// Regular expression matching using the regex crate
    Regex { regex: Regex },
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
    fn new(pattern: &str, ignore_case: bool, use_regex: bool) -> Result<Self> {
        if use_regex {
            let regex_pattern = if ignore_case {
                format!("(?i){}", pattern)
            } else {
                pattern.to_string()
            };
            let regex = Regex::new(&regex_pattern)
                .context("Invalid regex pattern")?;
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
    fn is_match(&self, line: &str) -> bool {
        match self {
            Matcher::Literal { pattern, ignore_case } => {
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

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: PathBuf,

    /// Perform case-insensitive matching
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    /// Show line numbers with output lines
    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,

    /// Interpret pattern as a regular expression
    #[arg(short = 'r', long = "regex")]
    regex: bool,
}

/// Searches through a reader line-by-line for lines matching the pattern.
///
/// # Arguments
///
/// * `reader` - Any type implementing Read (files, strings, etc.)
/// * `matcher` - The Matcher to use for pattern matching
///
/// # Returns
///
/// Returns a Result containing a Vec of SearchMatch structs, or an error if reading fails.
fn search_lines<R: Read>(reader: R, matcher: &Matcher) -> Result<Vec<SearchMatch>> {
    let buf_reader = BufReader::new(reader);
    let mut matches = Vec::new();

    for (line_number, line) in buf_reader.lines().enumerate() {
        let content = line?;
        if matcher.is_match(&content) {
            matches.push(SearchMatch {
                line_number: line_number + 1,  // 1-based indexing
                content,
            });
        }
    }

    Ok(matches)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let matcher = Matcher::new(&args.pattern, args.ignore_case, args.regex)?;
    let matches = search_lines(file, &matcher)?;

    for search_match in matches {
        if args.line_numbers {
            println!("{}:{}", search_match.line_number, search_match.content);
        } else {
            println!("{}", search_match.content);
        }
    }

    Ok(())
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