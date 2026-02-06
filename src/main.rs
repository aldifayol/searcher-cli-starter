//! Searcher CLI - Command-line interface for text search
//!
//! This is the binary executable that provides a CLI wrapper around
//! the searcher library functionality.

use anyhow::{Context, Result};
use clap::Parser;
use searcher_cli_starter::{search_lines, Matcher};
use std::fs::File;
use std::path::PathBuf;

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
