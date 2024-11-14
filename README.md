# Rustree

A simple command-line tool for managing and visualizing Git branches, written in Rust.

## Features

- List branches with detailed information (last commit date, author)
- Visualize branch relationships
- Clean up stale branches with configurable thresholds
- Protected branches support
- Customizable configuration

## Installation

```bash
cargo install rustree
```

## Usage

```bash
# List all branches
rtree list

# List branches sorted by date
rtree list --sort-by-date

# Visualize branch relationships
rtree visualize

# Include remote branches in visualization
rtree visualize --include-remote

# Clean up stale branches (dry run)
rtree cleanup --dry-run

# Clean up branches older than 60 days
rtree cleanup --days 60
```

## Configuration

The tool uses a configuration file located at `~/.config/rustree/config.json`. You can customize:

- Protected branches that won't be deleted
- Default remote name
- Maximum branch age threshold

## License

## License

This project is licensed under the MIT License. See [LICENSE](https://github.com/imperivox/Rustree/blob/main/LICENSE) for more details.
