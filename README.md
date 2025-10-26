# Git Team Stats

A fast and beautiful command-line tool for analyzing Git repository statistics, built with Rust.

## Features

- **Contributor Statistics**: Detailed breakdown of commits, additions, deletions per contributor
- **Time-based Analysis**: Visualize commit patterns by hour and day of week
- **File Change Frequency**: Identify the most frequently modified files
- **Comprehensive Reports**: Generate weekly or monthly team reports
- **Beautiful Output**: Colored tables and charts for easy reading
- **Fast Performance**: Built with Rust for maximum speed

## Installation

### From Source

```bash
cargo install --path .
```

### Development Build

```bash
cargo build --release
```

The binary will be available at `target/release/git-team-stats`.

## Configuration

Git Team Stats supports configuration via environment variables using a `.env` file.

### Setup Configuration

1. Copy the example configuration file:

```bash
cp .env.example .env
```

2. Edit `.env` to customize your settings:

```bash
# Repository Path
GIT_REPO_PATH=/path/to/your/repository

# Team Email Domains (comma-separated)
GIT_TEAMS=company.com,partner.org

# Default Analysis Period (days)
DEFAULT_DAYS=30
```

### Configuration Options

#### `GIT_REPO_PATH`
Specify the default Git repository path to analyze.
- If not set, uses current directory or `--repo` CLI argument
- Example: `GIT_REPO_PATH=/Users/john/projects/myapp`

#### `GIT_TEAMS`
Filter contributors by email domain or pattern (comma-separated).
- If not set, includes all contributors
- Examples:
  - `GIT_TEAMS=company.com` - Only @company.com emails
  - `GIT_TEAMS=team1,team2` - Emails containing "team1" or "team2"
  - `GIT_TEAMS=acme.com,partner.org` - Multiple domains

#### `DEFAULT_DAYS`
Default number of days to analyze when not specified via CLI.
- If not set, uses command-specific defaults
- Examples:
  - `DEFAULT_DAYS=7` - Last week
  - `DEFAULT_DAYS=30` - Last month
  - `DEFAULT_DAYS=90` - Last quarter
  - `DEFAULT_DAYS=0` - All time

### Configuration Priority

Settings are applied in this order (later overrides earlier):
1. `.env` file configuration
2. Command-line arguments

Example: If `.env` has `DEFAULT_DAYS=30` but you run `git-team-stats summary --days 7`, it will use 7 days.

## Usage

### Basic Commands

#### Show Team Summary

```bash
git-team-stats summary
```

Default period is last 30 days. Use `--days` to change:

```bash
git-team-stats summary --days 7
```

#### Contributor Statistics

```bash
git-team-stats contributors
```

Shows detailed statistics for each contributor including:
- Total commits
- Lines added and deleted
- Files changed
- Net change (additions - deletions)

#### Time-based Analysis

```bash
git-team-stats time-analysis
```

Displays beautiful bar charts showing:
- Commits by hour of day (24-hour format)
- Commits by day of week

#### File Change Frequency

```bash
git-team-stats files --top 20
```

Lists the most frequently changed files with:
- Number of changes
- Number of unique contributors

#### Generate Reports

```bash
# Weekly report (last 7 days)
git-team-stats report --period weekly

# Monthly report (last 30 days)
git-team-stats report --period monthly
```

Comprehensive report including all statistics in one view.

### Advanced Options

#### Analyze a Different Repository

```bash
git-team-stats --repo /path/to/repo summary
```

#### Filter by Time Period

All commands support the `--days` flag:

```bash
git-team-stats contributors --days 90  # Last 90 days
git-team-stats files --days 0          # All time
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── cli.rs               # CLI argument definitions
├── stats/               # Statistics analysis modules
│   ├── mod.rs          # Common utilities
│   ├── contributor.rs  # Contributor statistics
│   ├── time.rs         # Time-based analysis
│   └── files.rs        # File change analysis
└── display/             # Output formatting modules
    ├── mod.rs
    └── format.rs       # Table and chart formatting
```

## Development

### Prerequisites

- Rust 1.70.0 or higher
- Git repository for testing

### Setup

```bash
# Clone or create the project
git init
git add .
git commit -m "Initial commit"

# Install dependencies
cargo build
```

### Code Quality

This project follows strict code quality standards:

#### Format Code

```bash
cargo fmt
```

#### Run Linter

```bash
cargo clippy
```

#### Run Tests

```bash
cargo test
```

### Development Workflow

Always follow these steps when making changes:

1. Edit code
2. Format: `cargo fmt`
3. Lint: `cargo clippy`
4. Build: `cargo build`
5. Test: `cargo test` (if tests exist)

See [.claude/rule.md](.claude/rule.md) for detailed development rules.

## Configuration Files

- **.rustfmt.toml**: Code formatting rules
- **clippy.toml**: Linter configuration
- **.editorconfig**: Editor settings for consistency

## Dependencies

- **clap**: Command-line argument parsing
- **git2**: Git repository interaction
- **colored**: Terminal color output
- **comfy-table**: Beautiful table formatting
- **chrono**: Date and time handling
- **serde**: Serialization (for future features)

## Example Output

### Contributor Statistics

```
📊 Contributor Statistics

┌────────────────────┬─────────┬───────────┬───────────┬───────┬───────┐
│ Contributor        ┆ Commits ┆ Additions ┆ Deletions ┆ Files ┆ Net   │
╞════════════════════╪═════════╪═══════════╪═══════════╪═══════╪═══════╡
│ John <john@ex.com> ┆      15 ┆      1250 ┆       320 ┆    42 ┆  +930 │
└────────────────────┴─────────┴───────────┴───────────┴───────┴───────┘
```

### Time Analysis

```
⏰ Time-based Commit Analysis

Commits by Hour:
10:00 │ █████████████████████████ (5)
11:00 │ ██████████████████████████████████████████████████ (10)
14:00 │ ████████████████████ (4)
```

## Use Cases

- **Team Retrospectives**: Analyze team activity over sprint periods
- **Code Reviews**: Identify frequently changed files that need attention
- **Performance Analysis**: Understand when your team is most productive
- **Onboarding**: Show new team members contribution patterns
- **Management Reports**: Generate weekly/monthly statistics for stakeholders

## Performance

Built with Rust for maximum performance:
- Analyzes repositories with 10,000+ commits in seconds
- Low memory footprint
- Optimized binary size with LTO and stripping

## Future Enhancements

Potential features for future releases:
- Export reports to JSON/CSV
- Integration with GitHub/GitLab APIs
- More visualization options
- Custom date ranges
- Branch comparison
- Trend analysis over time

## License

MIT

## Contributing

Contributions are welcome! Please ensure all code:
- Passes `cargo fmt`
- Passes `cargo clippy` without warnings
- Builds successfully with `cargo build`
- Includes appropriate tests

## Author

Built with Rust for team development workflows.
