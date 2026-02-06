use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line?;
        if content.contains(&args.pattern) {
            println!("{}", content);
        }
    }

    Ok(())
}