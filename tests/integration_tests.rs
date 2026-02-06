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
