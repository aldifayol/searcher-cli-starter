# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-07

### Added
- Case-insensitive search with `-i` / `--ignore-case` flag
  - Matches patterns regardless of capitalization
  - Works with both literal strings and regular expressions
- Line number display with `-n` / `--line-numbers` flag
  - Shows 1-based line numbers in format `N:content`
  - Useful for locating matches in files
- Regular expression support with `-r` / `--regex` flag
  - Full regex syntax support via Rust's regex crate
  - Includes anchors, character classes, quantifiers, and more
- Comprehensive test coverage
  - 20 unit tests covering core functionality
  - 21 integration tests covering CLI behavior
  - Tests for all feature combinations
- Complete documentation suite
  - README.md with extensive usage examples
  - CHANGELOG.md for version tracking
  - CONTRIBUTING.md for contributor guidelines
  - ARCHITECTURE.md for technical details
  - EXAMPLES.md with practical use cases
  - API.md for internal documentation
- `SearchMatch` struct for structured match representation
- `Matcher` enum for flexible pattern matching strategies
- Detailed inline documentation with doc comments

### Changed
- Refactored `search_lines` function to return `Vec<SearchMatch>` instead of `Vec<String>`
  - Enables line number tracking
  - Provides foundation for future enhancements
- Updated internal architecture to separate matching logic from I/O
- Improved error messages for invalid regex patterns
- Enhanced `Cargo.toml` with comprehensive package metadata

### Performance
- Regex patterns are compiled once before search loop (significant performance improvement)
- Maintained efficient buffered I/O for line-by-line reading
- Zero-cost abstractions preserve original performance characteristics

### Documentation
- Added comprehensive inline documentation for all public types
- Created extensive README with usage examples
- Documented all command-line flags and options
- Included regex syntax reference
- Added contributing guidelines and architecture documentation

## [0.1.0] - 2026-02-07

### Added
- Initial release with basic search functionality
- File reading and pattern matching
- Case-sensitive literal string search
- Error handling for missing files with helpful messages
- Command-line interface using clap
- Basic test coverage (5 unit tests, 6 integration tests)
- Support for reading any file path
- Line-by-line streaming for memory efficiency

### Technical Details
- Built with Rust 2024 edition
- Uses clap 4.4 for CLI parsing
- Uses anyhow 1.0 for error handling
- Implements buffered I/O with `BufReader`

---

## Release Notes

### Upgrading from 0.1.0 to 0.2.0

Version 0.2.0 is fully backward compatible with 0.1.0. All existing command-line usage will continue to work unchanged:

```bash
# This command works identically in both versions
searcher "pattern" file.txt
```

New features are opt-in via flags:

```bash
# New in 0.2.0 - case-insensitive search
searcher -i "pattern" file.txt

# New in 0.2.0 - line numbers
searcher -n "pattern" file.txt

# New in 0.2.0 - regex support
searcher -r "pattern.*" file.txt
```

### Breaking Changes

None. Version 0.2.0 maintains full backward compatibility with 0.1.0.

### Migration Guide

No migration needed. Existing usage patterns continue to work without modification.

---

[0.2.0]: https://github.com/yourusername/searcher-cli-starter/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yourusername/searcher-cli-starter/releases/tag/v0.1.0
