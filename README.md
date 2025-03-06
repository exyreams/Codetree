# Codetree

Codetree is a powerful Rust-based command-line tool that generates a comprehensive overview of your project's file structure, code statistics, and contents. It intelligently analyzes your codebase, providing valuable insights while automatically protecting sensitive information.

## Features

- **Intelligent Project Detection**: Automatically identifies project types (Rust, Node.js, Python, Java, etc.) and adjusts analysis accordingly
- **Comprehensive Code Statistics**: Tracks total lines, code lines, comments, blank lines, and more
- **File Type Analysis**: Provides breakdowns of files and lines by language/extension
- **Environment Variable Protection**: Automatically detects and protects sensitive information
- **Visual File Tree**: Generates a clear, hierarchical view of your project structure
- **Complete Source Extraction**: Compiles all source code into a single, searchable document
- **Smart Build Directory Exclusion**: Automatically excludes build artifacts based on project type

## Why Use Codetree?

Sharing your codebase with AI assistants or collaborators typically requires tedious manual copying of files and explaining directory structures. Codetree eliminates this friction by:

1. Automatically generating a complete representation of your project structure
2. Collecting all source code into a single, well-organized document
3. Providing insightful statistics about your codebase
4. Protecting sensitive information like API keys and credentials

Whether you're seeking code review from an LLM, sharing with colleagues, or documenting your project, Codetree streamlines the process while ensuring security.

## Installation

### Prerequisites

- Rust and Cargo installed on your system ([install from rust-lang.org](https://www.rust-lang.org/tools/install))

### Method 1: Install from crates.io

```bash
cargo install codetree
```

### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/exyreams/Codetree.git
cd Codetree

# Build the project
cargo build --release

# The binary will be available at ./target/release/codetree
```

## Usage

### Basic Usage

To analyze the current directory:

```bash
codetree
```

To analyze a specific directory:

```bash
codetree /path/to/your/project
```

### Windows-specific Example

```bash
codetree.exe C:\path\to\your\project
```

### Output

Codetree will generate a `codetree.txt` file in the target directory containing:

1. Project type detection information
2. A visual representation of the project's file structure
3. Comprehensive code statistics
4. File type breakdown
5. The contents of each file (with sensitive information protected)

## Example Output

```
Project File Tree:

Detected Project Types: Rust, JavaScript/Node.js
Auto-excluded build directories: target, node_modules, dist, build

├── src/
│   ├── main.rs
│   └── utils/
│       └── helpers.rs
├── tests/
│   └── integration_test.rs
└── Cargo.toml

Project Statistics:
==================
Total Files: 4
Total Lines of Code: 235
  - Code Lines: 198 (84.3%)
  - Comment Lines: 22 (9.4%)
  - Blank Lines: 15 (6.3%)
Total Size: 8.32 KB

Files by Type:
  .rs: 3 files, 215 lines
  .toml: 1 file, 20 lines

Project Codes:
...
```

## Security Features

Codetree automatically protects sensitive information by:

1. Detecting common environment variable files (`.env`, etc.)
2. Identifying files with sensitive keywords (API keys, tokens, passwords)
3. Hiding the contents of these files in the output while maintaining statistics

## Customization

You can modify the source code to:

- Add or remove directories from the exclusion list
- Customize sensitive file detection patterns
- Adjust the project type detection rules

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.