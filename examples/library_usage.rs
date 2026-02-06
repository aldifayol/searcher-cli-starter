//! Example of using searcher as a library in your own Rust project
//!
//! Run this example with:
//! ```
//! cargo run --example library_usage
//! ```

use searcher_cli_starter::{search_lines, Matcher, SearchMatch};
use std::fs::File;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Searcher Library Usage Examples ===\n");

    // Example 1: Search in-memory strings
    example_1_in_memory_search()?;

    // Example 2: Case-insensitive search
    example_2_case_insensitive()?;

    // Example 3: Regex search
    example_3_regex_search()?;

    // Example 4: Search a file
    example_4_file_search()?;

    // Example 5: Process results
    example_5_process_results()?;

    Ok(())
}

fn example_1_in_memory_search() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Basic In-Memory Search");
    println!("{}", "-".repeat(40));

    let text = "hello world\nrust is great\nhello rust\nfarewell";
    let cursor = Cursor::new(text);

    let matcher = Matcher::new("hello", false, false)?;
    let results = search_lines(cursor, &matcher)?;

    println!("Searching for 'hello' in text:");
    for result in results {
        println!("  Line {}: {}", result.line_number, result.content);
    }
    println!();

    Ok(())
}

fn example_2_case_insensitive() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Case-Insensitive Search");
    println!("{}", "-".repeat(40));

    let text = "Rust is great\nRUST programming\nrust language";
    let cursor = Cursor::new(text);

    let matcher = Matcher::new("rust", true, false)?;
    let results = search_lines(cursor, &matcher)?;

    println!("Searching for 'rust' (case-insensitive):");
    println!("Found {} matches:", results.len());
    for result in results {
        println!("  Line {}: {}", result.line_number, result.content);
    }
    println!();

    Ok(())
}

fn example_3_regex_search() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Regular Expression Search");
    println!("{}", "-".repeat(40));

    let text = "error: Connection failed\nwarning: Retry attempt\nerror: Timeout occurred";
    let cursor = Cursor::new(text);

    let matcher = Matcher::new("^error:", false, true)?;
    let results = search_lines(cursor, &matcher)?;

    println!("Searching for lines starting with 'error:':");
    for result in results {
        println!("  Line {}: {}", result.line_number, result.content);
    }
    println!();

    Ok(())
}

fn example_4_file_search() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: File Search");
    println!("{}", "-".repeat(40));

    // Try to open the sample file
    match File::open("tests/fixtures/sample.txt") {
        Ok(file) => {
            let matcher = Matcher::new("Rust", false, false)?;
            let results = search_lines(file, &matcher)?;

            println!("Searching for 'Rust' in sample.txt:");
            println!("Found {} matches:", results.len());
            for result in results {
                println!("  Line {}: {}", result.line_number, result.content);
            }
        }
        Err(_) => {
            println!("(Sample file not found - skipping file search example)");
        }
    }
    println!();

    Ok(())
}

fn example_5_process_results() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 5: Processing Search Results");
    println!("{}", "-".repeat(40));

    let text = "error: code 404\ninfo: success\nerror: code 500\nwarning: slow query";
    let cursor = Cursor::new(text);

    let matcher = Matcher::new("error", false, false)?;
    let results = search_lines(cursor, &matcher)?;

    println!("Processing error lines:");

    // Filter results
    let critical_errors: Vec<&SearchMatch> = results
        .iter()
        .filter(|m| m.content.contains("500"))
        .collect();

    println!("Total errors: {}", results.len());
    println!("Critical errors (500): {}", critical_errors.len());

    // Extract line numbers
    let error_line_numbers: Vec<usize> = results.iter().map(|m| m.line_number).collect();
    println!("Error occurred on lines: {:?}", error_line_numbers);

    // Count by content
    for result in &results {
        let code = if result.content.contains("404") {
            "404 Not Found"
        } else if result.content.contains("500") {
            "500 Internal Server Error"
        } else {
            "Unknown"
        };
        println!("  Line {}: {}", result.line_number, code);
    }
    println!();

    Ok(())
}
