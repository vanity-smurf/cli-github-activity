# GitHub Activity CLI

A command-line tool to fetch and display GitHub user activity statistics.

## Features

- Fetch recent GitHub events for any public user
- Display summarized activity counts by event type
- Simple and lightweight interface
- Built with Rust for performance

## Installation

### Prerequisites

- Rust and Cargo installed (version 1.70+ recommended)

### From Source

```bash
git clone https://github.com/vanity-smurf/cli-github-activity.git
cd cli-github-activity
cargo install --path .
```

## Usage

Basic command:
```bash
github-activity <username>
```

Example:
```bash
github-activity torvalds
```

Sample output:
```text
Hi, torvalds
{
    "PushEvent": 15,
    "IssuesEvent": 3,
    "WatchEvent": 8
}
```

## Supported Event Types

- `PushEvent`: Code pushes to repositories
- `IssuesEvent`: Issue-related activities
- `WatchEvent`: Repository starring events

## Rate Limits

GitHub API has rate limits:
- 60 requests per hour for unauthenticated requests
- 5000 requests per hour with authentication

To use with a personal access token:
```bash
export GITHUB_TOKEN="your_personal_access_token"
github-activity <username>
```

## Building from Source

Build optimized release:
```bash
cargo build --release
```

Run directly:
```bash
./target/release/github-activity <username>
```

## Troubleshooting

If you encounter issues:
- Check your internet connection
- Verify the username exists
- Ensure you haven't exceeded rate limits
- For authentication issues, verify your token has proper permissions

## Dependencies

- [clap](https://crates.io/crates/clap): Command line argument parsing
- [reqwest](https://crates.io/crates/reqwest): HTTP client
- [tokio](https://crates.io/crates/tokio): Async runtime
- [serde](https://crates.io/crates/serde): JSON serialization/deserialization
