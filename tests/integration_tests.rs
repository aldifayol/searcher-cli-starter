#![allow(deprecated)]

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("pattern")
        .arg("nonexistent/file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));
}

#[test]
fn test_search_finds_matches() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("Rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"))
        .stdout(predicate::str::contains("Hello world from Rust"))
        .stdout(predicate::str::contains("Rust makes systems programming accessible"));
}

#[test]
fn test_search_no_matches() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("nonexistent")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_search_case_sensitive() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_search_partial_match() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("line")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.lines().count();

    assert_eq!(line_count, 2);
    assert!(stdout.contains("Another line without the search term"));
    assert!(stdout.contains("Final line of the test file"));
}

#[test]
fn test_with_temporary_file() {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("searcher_test.txt");

    fs::write(&temp_file, "line one\nline two\nline three").unwrap();

    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("two")
        .arg(&temp_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("line two"));

    fs::remove_file(temp_file).ok();
}

// Case-insensitive tests
#[test]
fn test_ignore_case_short_flag() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-i")
        .arg("rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"))
        .stdout(predicate::str::contains("Hello world from Rust"))
        .stdout(predicate::str::contains("Rust makes systems programming accessible"));
}

#[test]
fn test_ignore_case_long_flag() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("--ignore-case")
        .arg("rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"));
}

#[test]
fn test_ignore_case_with_uppercase_pattern() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-i")
        .arg("RUST")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"))
        .stdout(predicate::str::contains("Hello world from Rust"));
}

// Line number tests
#[test]
fn test_line_numbers_short_flag() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-n")
        .arg("Rust")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2:Rust is a systems programming language"));
    assert!(stdout.contains("3:Hello world from Rust"));
    assert!(stdout.contains("5:Rust makes systems programming accessible"));
}

#[test]
fn test_line_numbers_long_flag() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("--line-numbers")
        .arg("Rust")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2:"));
    assert!(stdout.contains("3:"));
    assert!(stdout.contains("5:"));
}

#[test]
fn test_line_numbers_format_correct() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-n")
        .arg("quick")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("1:"));
}

// Regex tests
#[test]
fn test_regex_basic_pattern() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-r")
        .arg("R.*t")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_regex_word_boundary() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-r")
        .arg(r"\bRust\b")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.lines().count();
    // Should match "Rust" but not "Rust's" or similar
    assert_eq!(line_count, 3);
}

#[test]
fn test_regex_character_class() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-r")
        .arg("[Rr]ust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_regex_invalid_pattern_fails() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-r")
        .arg("[unclosed")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid regex pattern"));
}

// Combination tests
#[test]
fn test_case_insensitive_and_line_numbers() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-i")
        .arg("-n")
        .arg("rust")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2:Rust is a systems programming language"));
    assert!(stdout.contains("3:Hello world from Rust"));
    assert!(stdout.contains("5:Rust makes systems programming accessible"));
}

#[test]
fn test_regex_and_line_numbers() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-r")
        .arg("-n")
        .arg("R.*t")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2:"));
    assert!(stdout.contains("Rust"));
}

#[test]
fn test_case_insensitive_and_regex() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("-i")
        .arg("-r")
        .arg("rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"));
}

#[test]
fn test_all_flags_combined() {
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    let output = cmd
        .arg("-i")
        .arg("-n")
        .arg("-r")
        .arg("rust")
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2:Rust is a systems programming language"));
    assert!(stdout.contains("3:Hello world from Rust"));
    assert!(stdout.contains("5:Rust makes systems programming accessible"));
}

#[test]
fn test_backward_compatibility() {
    // Ensure basic search still works without any flags
    let mut cmd = Command::cargo_bin("searcher").unwrap();
    cmd.arg("Rust")
        .arg("tests/fixtures/sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust is a systems programming language"));
}
