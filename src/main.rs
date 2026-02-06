use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: PathBuf,
}

fn search_lines<R: Read>(reader: R, pattern: &str) -> Result<Vec<String>> {
    let buf_reader = BufReader::new(reader);
    let mut matches = Vec::new();

    for line in buf_reader.lines() {
        let content = line?;
        if content.contains(pattern) {
            matches.push(content);
        }
    }

    Ok(matches)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let matches = search_lines(file, &args.pattern)?;

    for line in matches {
        println!("{}", line);
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

        let results = search_lines(cursor, "hello").unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "hello world");
        assert_eq!(results[1], "hello rust");
    }

    #[test]
    fn test_search_no_matches() {
        let input = "foo\nbar\nbaz";
        let cursor = Cursor::new(input);

        let results = search_lines(cursor, "nonexistent").unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_case_sensitive() {
        let input = "Hello World\nhello world";
        let cursor = Cursor::new(input);

        let results = search_lines(cursor, "hello").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "hello world");
    }

    #[test]
    fn test_search_empty_input() {
        let input = "";
        let cursor = Cursor::new(input);

        let results = search_lines(cursor, "anything").unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_partial_match() {
        let input = "testing\ntest\ncontest";
        let cursor = Cursor::new(input);

        let results = search_lines(cursor, "test").unwrap();

        assert_eq!(results.len(), 3);
    }
}