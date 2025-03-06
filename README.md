# Codetree

Codetree is a powerful Rust-based command-line tool that generates a comprehensive overview of your project's file structure, code statistics, and contents. It intelligently analyzes your codebase, detecting frameworks and project types while automatically protecting sensitive information.

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/exyreams/Codetree)](https://github.com/exyreams/Codetree/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


░█████╗░░█████╗░██████╗░███████╗████████╗██████╗░███████╗███████╗
██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔════╝
██║░░╚═╝██║░░██║██║░░██║█████╗░░░░░██║░░░██████╔╝█████╗░░█████╗░░
██║░░██╗██║░░██║██║░░██║██╔══╝░░░░░██║░░░██╔══██╗██╔══╝░░██╔══╝░░
╚█████╔╝╚█████╔╝██████╔╝███████╗░░░██║░░░██║░░██║███████╗███████╗
░╚════╝░░╚════╝░╚═════╝░╚══════╝░░░╚═╝░░░╚═╝░░╚═╝╚══════╝╚══════╝


## 🚀 What's New in v2.0.0

Our latest release includes:

- **Framework Detection**: Automatically identifies 20+ frameworks including React, Vue, Angular, Next.js, Three.js, Django, and more
- **Enhanced Project Type Detection**: Detects Rust, Node.js, Python, Java, .NET, Go, Ruby, and PHP projects
- **Build Directory Auto-Exclusion**: Intelligently excludes build directories based on project type
- **Security Enhancements**: Detects and protects sensitive information like API keys and credentials
- **Improved Statistics**: Detailed breakdown of code, comments, and blank lines

**[Check out the latest release](https://github.com/exyreams/Codetree/releases/latest)**

## 🔍 Why Use Codetree?

Sharing your codebase with AI assistants or collaborators typically requires tedious manual copying of files and explaining directory structures. Codetree eliminates this friction by:

1. Generating a complete representation of your project structure
2. Collecting all source code into a single, well-organized document
3. Providing insightful statistics about your codebase
4. Protecting sensitive information like API keys and credentials
5. Detecting frameworks and project types automatically

Perfect for:
- Getting AI assistance with your code
- Sharing project overviews with colleagues
- Documenting your codebase
- Analyzing project statistics

## 📦 Installation

### Option 1: Download Prebuilt Binaries

**[Download the latest release](https://github.com/exyreams/Codetree/releases/latest)**

Choose the appropriate binary for your system:
- `codetree-v2.0.0-windows.exe` for Windows
- `codetree-v2.0.0-linux` for Linux

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/exyreams/Codetree.git
cd Codetree

# Build the project
cargo build --release

# The binary will be in target/release/
```

### Option 3: Install from crates.io

```bash
cargo install codetree
```

## 🖥️ Usage

### Basic Usage

To analyze the current directory:

```bash
codetree
```

To analyze a specific directory:

```bash
codetree /path/to/your/project
```

### Windows Examples

```bash
# Current directory
codetree.exe

# Specific directory
codetree.exe C:\path\to\your\project
```

## ✨ Features

### Framework and Project Detection

Codetree intelligently detects:

- **Frontend Frameworks**: React, Vue.js, Angular, Next.js, Three.js, Svelte, etc.
- **Backend Frameworks**: Express.js, Django, Flask, Spring Boot, Rails, Laravel, etc.
- **Project Types**: Rust, Node.js, Python, Java, .NET, Go, Ruby, PHP
- **UI Libraries**: Tailwind CSS, Material UI, Bootstrap, Chakra UI
- **State Management**: Redux, MobX
- **Testing Frameworks**: Jest, Cypress, Pytest

### Comprehensive Code Statistics

- Total files and lines of code
- Breakdown of code, comment, and blank lines
- Files and lines by language/extension
- Project size metrics

### Security Features

Codetree automatically detects and protects sensitive files like:
- Environment variables (`.env` files)
- API keys and tokens
- Configuration files with credentials
- Connection strings

### Smart Build Directory Exclusion

Based on your project type, Codetree automatically excludes:
- `node_modules` for JavaScript/Node.js projects
- `target` for Rust projects
- `__pycache__` and `.venv` for Python projects
- `bin` and `obj` for .NET projects
- And many more

## 📋 Output

Codetree generates a `codetree.txt` file in the analyzed directory containing:

1. Project type and framework detection information
2. A visual representation of the project's file structure
3. Comprehensive code statistics
4. The contents of each file (with sensitive information protected)

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin my-feature`
5. Submit a pull request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

*Made with ❤️ by [exyreams](https://github.com/exyreams)*