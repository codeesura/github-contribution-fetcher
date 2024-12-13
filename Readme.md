# GitHub Contribution Fetcher

A Rust command-line tool that fetches and displays a user's merged GitHub pull requests in a nicely formatted table.

## Features

- 📊 Fetches all merged pull requests using GitHub's Search API
- 📋 Displays results in a clean ASCII table format
- 🔍 Shows repository name, PR link, and merge date
- 🎨 Uses colored output for better readability
- ⚡ Asynchronous request handling for better performance
- 📂 Supports filtering by repositories listed in a `repos.txt` file

## Prerequisites

- Rust 1.76 or higher
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/codeesura/github-contribution-fetcher.git
cd github-contribution-fetcher
```

## Usage

Run the program with a GitHub username:

```bash
cargo run <github-username> [repos.txt]
```

### Example:

```bash
cargo run octocat
```

With a `repos.txt` file:

```bash
cargo run octocat repos.txt
```

## Output Format

The tool displays results in a table with the following columns:

- Repository name
- Pull request URL
- Merge date (or creation date if not merged)

## Dependencies

- reqwest - HTTP client
- tokio - Async runtime
- serde - JSON serialization
- colored - Terminal colors

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## License

This project is available under the MIT license.
