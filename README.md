# File System Analyzer & Finder

A command-line tool for analyzing directory structures and searching files with advanced filters.

## Features

**Analyzer Module**  
📂 Directory analysis:
- Total size/file count/folder count/symlink count
- Top N largest files & folders
- Duplicate file detection (SHA-256 hash based)
- Configurable depth/size filters
- Ignore patterns support

**Search Module**  
🔍 Advanced file search:
- Name patterns (glob syntax)
- Content matching
- Size ranges (min/max)
- Date modified filters
- Combined search criteria

## Installation

1. Ensure Rust toolchain is installed ([rustup.rs](https://rustup.rs/))
2. Clone repository
3. Build and install:
```bash
cargo install --path .
```

## Usage

### Basic Command Structure
```bash
fs-tool <COMMAND> [OPTIONS]
```

### Commands

#### Analyze Directory
```bash
fs-tool analyze [PATH] [OPTIONS]
```

**Options**:
| Option           | Description                          | Default      |
|------------------|--------------------------------------|--------------|
| `-d, --max-depth`  | Maximum directory depth              | Unlimited    |
| `-n, --top-n`      | Number of largest items to show      | 5            |
| `-L, --follow-symlinks` | Follow symbolic links              | false        |
| `-s, --min-size`    | Minimum file size (e.g., 10MB)       | 0            |
| `-D, --duplicates`  | Detect duplicate files              | false        |
| `-i, --ignore`      | Ignore patterns (comma-separated)    | None         |

**Example**:
```bash
fs-tool analyze ~/Documents -n 10 -s 5MB -D -i "temp*,*.tmp"
```

#### Search Files
```bash
fs-tool search [PATH] [OPTIONS]
```

**Options**:
| Option                 | Description                          |
|------------------------|--------------------------------------|
| `-N, --name-pattern`   | File name patterns (glob, comma-sep) |
| `-c, --content-pattern`| Search text in files                 |
| `-a, --modified-after` | Last modified after (YYYY-MM-DD)     |
| `-b, --modified-before`| Last modified before (YYYY-MM-DD)    |
| `--min`                | Minimum file size (bytes)            |
| `--max`                | Maximum file size (bytes)            |

**Example**:
```bash
fs-tool search . -N "*.log,*.txt" -c "ERROR" -a 2024-01-01 --min 1024
```

## Output Samples

### Analysis Report
```
📊 File System Analysis Report
📂 Path: /home/user/Documents
⏱️  Scan completed in 0.8 seconds

Directory Summary:
├── Total size: 2.4 GB
├── Files: 1 234
├── Folders: 45
└── Symlinks: 3

Largest Directories:
1. .../Documents/Projects        1.2 GB
2. .../Documents/Archives        800 MB

Largest Files:
1. .../project/video.mp4         650 MB
2. .../backup.zip                320 MB

Duplicates:
  Group #1 (Size= 150MB, Hash= a1b2c3):
    1. .../file1.txt
    2. .../copy/file1.txt
```

### Search Results
```
🔍 Search Results (3 matches):

report.log
├── Size: 2.4 MB
└── Modified: 2024-03-15 14:30:00

📊 Summary:
├── Files found: 3
├── Total size: 5.6 MB
└── Search time: 0.4s
```

## Error Handling

Common errors include:
- `Path not found`: Verify directory exists
- `Invalid pattern`: Check glob syntax
- `Permission denied`: Run with appropriate privileges

## Limitations

- Large file hashing may impact performance
- Content search is line-based (no regex)
- Date filters use system timezone

---

**Note**: For developer documentation, see inline code comments and module structure.