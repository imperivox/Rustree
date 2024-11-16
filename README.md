# Rustree

A simple command-line tool for managing and visualizing Git branches, written in Rust.

## Features

- **Branch Listing**
- **Branch Visualization**
- **Stale Branch Cleanup**
- **Flexible Configuration**
- **Interactive CLI**

## Installation

```bash
git clone https://github.com/imperivox/rustree.git
cd rustree
cargo build --release
```

## Usage

### Command-Line Interface

```bash
# Display help information
rtree --help

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

### Interactive Mode

Simply run `rtree` without any arguments to enter interactive mode:

```bash
rtree
```

This will present a menu-driven interface with the following options:

- List all branches
- List branches (sorted by date)
- Visualize branch relationships
- Visualize branch relationships (including remote)
- Clean up old branches
- Exit

## Configuration

Rustree can be configured through a JSON configuration file located at:
- Linux/macOS: `~/.config/rustree/config.json`
- Windows: `%APPDATA%\rustree\config.json`. If the file doesn't exist, default values will be used.

### Default Configuration

```json
{
  "default_remote": "main",
  "protected_branches": ["origin", "master", "develop"],
  "max_branch_age_days": 30
}
```

### Configuration Options

| Option                | Description                              | Default                           |
| --------------------- | ---------------------------------------- | --------------------------------- |
| `default_remote`      | Default remote name                      | `"main"`                          |
| `protected_branches`  | Branches that won't be deleted           | `["origin", "master", "develop"]` |
| `max_branch_age_days` | Default age threshold for stale branches | `30`                              |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/imperivox/Rustree/blob/main/LICENSE) file for details.
