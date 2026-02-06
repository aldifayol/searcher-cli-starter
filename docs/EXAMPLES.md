# Searcher CLI - Extended Examples

This document provides comprehensive examples of using Searcher CLI for various real-world scenarios.

## Table of Contents

- [Basic Search](#basic-search)
- [Case-Insensitive Search](#case-insensitive-search)
- [Line Numbers](#line-numbers)
- [Regular Expressions](#regular-expressions)
- [Combining Features](#combining-features)
- [Real-World Use Cases](#real-world-use-cases)
- [Common Patterns](#common-patterns)

## Basic Search

### Simple Text Match

Find all lines containing "error":

```bash
searcher "error" logfile.txt
```

Output:
```
Connection error occurred
Timeout error in handler
Database error: connection refused
```

### Exact Phrase

Search for a specific phrase:

```bash
searcher "user logged in" access.log
```

### Search in Code

Find function calls:

```bash
searcher "println!" main.rs
```

## Case-Insensitive Search

### Using Short Flag

Find "rust" regardless of case:

```bash
searcher -i "rust" article.txt
```

Matches: "Rust", "RUST", "rust", "RuSt"

### Using Long Flag

More explicit form:

```bash
searcher --ignore-case "python" docs.md
```

### Case-Insensitive in Logs

Find error messages with any capitalization:

```bash
searcher -i "warning" system.log
```

Matches: "WARNING", "Warning", "warning"

## Line Numbers

### Basic Line Numbers

Show where matches occur:

```bash
searcher -n "TODO" src/main.rs
```

Output:
```
15:// TODO: Implement error handling
42:// TODO: Add tests
89:// TODO: Refactor this function
```

### Find and Fix

Use line numbers to locate issues:

```bash
searcher -n "FIXME" src/*.rs
```

### Debugging

Find where a variable is used:

```bash
searcher -n "user_id" app.py
```

## Regular Expressions

### Basic Patterns

#### Match Lines Starting With Pattern

```bash
searcher -r "^ERROR" logfile.txt
```

Matches:
```
ERROR: Connection failed
ERROR: Timeout occurred
```

Does not match:
```
[2024] ERROR: Something  # Doesn't start with ERROR
```

#### Match Lines Ending With Pattern

```bash
searcher -r "failed$" logfile.txt
```

Matches:
```
Connection attempt failed
Database query failed
```

### Wildcards and Quantifiers

#### Match Any Character

```bash
searcher -r "h.t" words.txt
```

Matches: "hat", "hot", "hit", "h@t"

#### Match Zero or More

```bash
searcher -r "colou?r" text.txt
```

Matches: "color", "colour"

#### Match One or More

```bash
searcher -r "be+t" words.txt
```

Matches: "bet", "beet", "beeet"
Does not match: "bt"

#### Match Specific Count

```bash
searcher -r "\\d{3}-\\d{4}" contacts.txt
```

Matches phone numbers: "123-4567"

### Character Classes

#### Match Character Set

```bash
searcher -r "[Rr]ust" article.txt
```

Matches: "Rust", "rust"

#### Match Range

```bash
searcher -r "[A-Z][a-z]+" text.txt
```

Matches capitalized words: "Hello", "World"

#### Match Digits

```bash
searcher -r "\\d+" data.txt
```

Matches any sequence of digits: "123", "4567"

#### Match Word Characters

```bash
searcher -r "\\w+@\\w+\\.\\w+" emails.txt
```

Matches simple email addresses: "user@example.com"

### Word Boundaries

#### Match Whole Words Only

```bash
searcher -r "\\brust\\b" text.txt
```

Matches: "rust"
Does not match: "trustworthy", "rust_lang"

#### Match Word Starting With Pattern

```bash
searcher -r "\\bpre\\w+" text.txt
```

Matches: "prefix", "prepare", "preview"

### Alternation

#### Match Multiple Patterns

```bash
searcher -r "error|warning|critical" system.log
```

Matches lines containing "error" OR "warning" OR "critical"

#### Match Variants

```bash
searcher -r "gray|grey" text.txt
```

Matches both "gray" and "grey"

### Groups and Capture

#### Match Repeated Patterns

```bash
searcher -r "(\\w+)\\s+\\1" text.txt
```

Matches repeated words: "the the", "and and"

### Advanced Patterns

#### Match IP Addresses

```bash
searcher -r "\\b\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\b" network.log
```

Matches: "192.168.1.1", "10.0.0.1"

#### Match URLs

```bash
searcher -r "https?://[\\w./-]+" webpage.html
```

Matches: "http://example.com", "https://site.org/path"

#### Match Dates (YYYY-MM-DD)

```bash
searcher -r "\\d{4}-\\d{2}-\\d{2}" logs.txt
```

Matches: "2024-02-07", "2023-12-25"

#### Match Time (HH:MM:SS)

```bash
searcher -r "\\d{2}:\\d{2}:\\d{2}" timestamps.txt
```

Matches: "14:30:45", "09:15:00"

## Combining Features

### Case-Insensitive with Line Numbers

Find TODO comments regardless of case:

```bash
searcher -i -n "todo" src/main.rs
```

Output:
```
15:// TODO: Implement feature
42:// Todo: Fix bug
89:// todo: refactor
```

### Regex with Line Numbers

Find error lines and show their locations:

```bash
searcher -r -n "^\\[ERROR\\]" application.log
```

Output:
```
23:[ERROR] Database connection failed
156:[ERROR] Authentication timeout
478:[ERROR] Memory allocation error
```

### Case-Insensitive Regex

Find status codes regardless of case:

```bash
searcher -i -r "status.*\\d{3}" api.log
```

Matches: "Status: 200", "STATUS: 404", "status code 500"

### All Flags Combined

Most powerful combination:

```bash
searcher -i -n -r "warning|error" system.log
```

Output:
```
12:WARNING: Low disk space
45:Error in authentication
78:CRITICAL ERROR: System failure
```

## Real-World Use Cases

### 1. Finding TODOs in a Codebase

```bash
# Find all TODOs with line numbers
searcher -i -n "todo" src/main.rs

# Find FIXMEs
searcher -i -n "fixme" src/*.rs
```

### 2. Analyzing Log Files

```bash
# Find all errors
searcher -i "error" /var/log/app.log

# Find errors with context (line numbers)
searcher -i -n "error" /var/log/app.log

# Find specific error codes
searcher -r "error.*\\b[45]\\d{2}\\b" /var/log/app.log
```

### 3. Debugging Application

```bash
# Find where a function is called
searcher -n "process_payment" src/main.rs

# Find variable usage
searcher -n "user_id" app.py

# Find API endpoints
searcher -r "^\\s*(get|post|put|delete)\\(" routes.js
```

### 4. Code Review

```bash
# Find print statements (often left in by accident)
searcher "console.log" src/*.js

# Find debug code
searcher -i "debug" src/*.py

# Find commented out code
searcher -r "^\\s*#.*code" src/*.py
```

### 5. Configuration Files

```bash
# Find specific settings
searcher "timeout" config.yml

# Find environment variables
searcher -r "^[A-Z_]+=" .env

# Find ports
searcher -r "port.*\\d+" config.json
```

### 6. Documentation Search

```bash
# Find function documentation
searcher -i "## Installation" README.md

# Find code examples
searcher -n "```rust" docs/*.md

# Find links
searcher -r "https?://" documentation.md
```

### 7. Data Processing

```bash
# Find email addresses
searcher -r "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}" contacts.txt

# Find phone numbers
searcher -r "\\d{3}-\\d{3}-\\d{4}" data.csv

# Find dates
searcher -r "\\d{4}-\\d{2}-\\d{2}" records.txt
```

### 8. Security Auditing

```bash
# Find potential API keys
searcher -r "api[_-]?key" config.txt

# Find hardcoded passwords (case-insensitive)
searcher -i -r "password.*=.*['\"]" src/*.js

# Find SQL queries (potential injection points)
searcher -r "SELECT.*FROM" src/*.php
```

### 9. Performance Analysis

```bash
# Find slow queries
searcher -n "duration.*[5-9]\\d{3}" slow_query.log

# Find memory warnings
searcher -i -n "memory" performance.log

# Find timeout errors
searcher -r "timeout.*\\d+ms" application.log
```

### 10. Version Control

```bash
# Find merge conflicts
searcher -n "<<<<<<< HEAD" src/*.rs

# Find branch references
searcher -r "feature/\\w+" git.log

# Find commit messages about bugs
searcher -i "fix.*bug" CHANGELOG.md
```

## Common Patterns

### Email Addresses

```bash
searcher -r "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}" file.txt
```

### Phone Numbers (US Format)

```bash
# Format: (123) 456-7890
searcher -r "\\(\\d{3}\\)\\s*\\d{3}-\\d{4}" contacts.txt

# Format: 123-456-7890
searcher -r "\\d{3}-\\d{3}-\\d{4}" contacts.txt
```

### URLs

```bash
searcher -r "https?://[\\w./?#-]+" webpage.html
```

### IPv4 Addresses

```bash
searcher -r "\\b\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\b" network.log
```

### MAC Addresses

```bash
searcher -r "([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}" devices.txt
```

### Credit Card Numbers (for testing)

```bash
searcher -r "\\b\\d{4}[- ]?\\d{4}[- ]?\\d{4}[- ]?\\d{4}\\b" transactions.txt
```

### Hashtags

```bash
searcher -r "#\\w+" social_media.txt
```

### Hex Colors

```bash
searcher -r "#[0-9A-Fa-f]{6}\\b" styles.css
```

### Markdown Headers

```bash
searcher -r "^#{1,6}\\s+.+" README.md
```

### Function Definitions (Python)

```bash
searcher -r "^\\s*def\\s+\\w+" script.py
```

### Import Statements (JavaScript)

```bash
searcher -r "^import.*from" module.js
```

### Environment Variables

```bash
searcher -r "^[A-Z_][A-Z0-9_]*=" .env
```

## Tips and Tricks

### 1. Escaping Special Characters

In regex mode, escape special characters:

```bash
# Find literal dots
searcher -r "\\." text.txt

# Find literal brackets
searcher -r "\\[\\]" text.txt

# Find literal dollar signs
searcher -r "\\$" prices.txt
```

### 2. Testing Patterns

Test patterns on small files first:

```bash
# Create test file
echo -e "test@example.com\\nuser@site.org" > test.txt

# Test pattern
searcher -r "[\\w.]+@[\\w.]+" test.txt
```

### 3. Combining with Shell Tools

```bash
# Count matches
searcher "error" log.txt | wc -l

# Save results
searcher -n "TODO" src/main.rs > todos.txt

# Process results
searcher -r "user_\\d+" data.txt | sort | uniq
```

### 4. Reading from Shell Variables

```bash
PATTERN="error"
searcher "$PATTERN" logfile.txt
```

### 5. Searching Multiple Patterns

Use regex alternation:

```bash
searcher -r "error|warning|critical|fatal" system.log
```

## Learning Resources

### Regex Resources

- [Regex101](https://regex101.com/) - Test regex patterns interactively
- [RegexOne](https://regexone.com/) - Interactive regex tutorial
- [Rust Regex Docs](https://docs.rs/regex/) - Official regex crate documentation

### Example Files

Create test files to practice:

```bash
# Create sample log
cat > sample.log << EOF
2024-02-07 10:30:45 [INFO] Server started
2024-02-07 10:31:12 [ERROR] Connection failed
2024-02-07 10:31:45 [WARNING] Retry attempt 1
2024-02-07 10:32:00 [INFO] Connection established
EOF

# Try patterns
searcher -r "^\\d{4}-\\d{2}-\\d{2}" sample.log
searcher -i -n "error|warning" sample.log
```

## Summary

Searcher CLI provides powerful text search capabilities through three main features:

1. **Case-insensitive search** (`-i`) - Find matches regardless of case
2. **Line numbers** (`-n`) - Locate matches in files
3. **Regular expressions** (`-r`) - Complex pattern matching

Combine these features to solve real-world problems efficiently!

---

For more examples and use cases, visit the [GitHub repository](https://github.com/yourusername/searcher-cli-starter).
