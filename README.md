<div align="center">
  
<pre>
░█████╗░░█████╗░██████╗░███████╗████████╗██████╗░███████╗███████╗
██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔════╝
██║░░╚═╝██║░░██║██║░░██║█████╗░░░░░██║░░░██████╔╝█████╗░░█████╗░░
██║░░██╗██║░░██║██║░░██║██╔══╝░░░░░██║░░░██╔══██╗██╔══╝░░██╔══╝░░
╚█████╔╝╚█████╔╝██████╔╝███████╗░░░██║░░░██║░░██║███████╗███████╗
░╚════╝░░╚════╝░╚═════╝░╚══════╝░░░╚═╝░░░╚═╝░░╚═╝╚══════╝╚══════╝
</pre>

</div>
<p align="center">
  <a href="https://github.com/exyreams/Codetree/releases/latest">
    <img src="https://img.shields.io/github/v/release/exyreams/Codetree" alt="GitHub release (latest by date)">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
  </a>
</p>

Codetree is a powerful Rust-based command-line tool that generates a comprehensive overview of your project's file structure, code statistics, and contents. It intelligently analyzes your codebase, detecting frameworks and project types while automatically protecting sensitive information.
## 🚀 Key Features

Codetree offers comprehensive codebase analysis with these powerful features:

- **Multiple Output Formats**: Generate reports in Text, JSON, Markdown, or interactive HTML formats
- **Interactive HTML Reports**: Beautiful, responsive web interface with collapsible sections, search functionality, and syntax highlighting
- **Advanced Framework Detection**: Automatically identifies 25+ frameworks including React, Vue, Angular, Next.js, Three.js, Django, Flask, Spring Boot, and more
- **Intelligent Project Type Detection**: Detects Rust, Node.js, Python, Java, .NET, Go, Ruby, and PHP projects with automatic build directory exclusion
- **Enhanced Security**: Detects and protects 15+ types of sensitive files including environment variables, API keys, and configuration files
- **Comprehensive Statistics**: 
  - Detailed breakdown of code, comments, and blank lines with percentage calculations
  - **Advanced Cross-Platform File Size Analysis**: Real disk usage detection that accounts for:
    - File system compression (Windows NTFS compression, etc.)
    - Sparse file handling on Unix systems
    - Accurate disk block allocation vs logical file size
    - Cross-platform compatibility with Windows API and Unix system calls
  - Dual size tracking: both filesystem size and UTF-8 content size
  - Average file size calculations and top 10 largest files identification
  - Statistics breakdown by file extension and programming language
  - **Enhanced Exclusion Tracking**: Comprehensive reporting of excluded content including:
    - Total size of all excluded directories and files
    - Detailed breakdown of excluded build directories (node_modules, target, etc.) with size and file count
    - Top 10 largest excluded directories sorted by size with exclusion reasons
    - Top 10 largest excluded files with individual file sizes
    - Complete visibility into what content is being filtered out for better project understanding
- **Smart Content Analysis**: Line-by-line analysis with comment detection for multiple programming languages
- **Structured Data Export**: JSON output for integration with CI/CD pipelines and automated analysis tools

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
- `codetree-v0.3.0-windows.exe` for Windows
- `codetree-v0.3.0-linux` for Linux

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

### Output Format Options

Codetree now supports multiple output formats:

```bash
# Generate interactive HTML report (recommended for viewing)
codetree --format html

# Generate JSON output for programmatic use
codetree --format json

# Generate Markdown report
codetree --format markdown

# Generate traditional text output (default)
codetree --format text
```

### Windows Examples

```bash
# Current directory with HTML output
codetree.exe --format html

# Specific directory with JSON output
codetree.exe C:\path\to\your\project --format json

# Traditional text output
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

- **File Analysis**: Total files count with detailed breakdown by extension
- **Line Analysis**: Complete breakdown of code, comment, and blank lines with percentage calculations
- **Size Metrics**: 
  - Total filesystem size vs content size comparison
  - Average file size calculations
  - Top 10 largest files identification
  - Size breakdown by file extension and language
- **Language Detection**: Automatic detection and statistics for multiple programming languages
- **Content Quality**: Code-to-comment ratio analysis and blank line percentage tracking

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

## 📋 Output Formats

Codetree generates reports in multiple formats, each optimized for different use cases:

### 📄 Text Format (Default)
Generates `codetree.txt` - the classic format perfect for AI assistants and simple sharing:
- Project type and framework detection information
- Visual directory tree structure
- Comprehensive code statistics
- All source code content with sensitive information protected

### 🌐 HTML Format (Interactive)
Generates `codetree.html` - a beautiful, interactive web report featuring:
- **Responsive Design**: Works perfectly on desktop and mobile
- **Collapsible Sections**: Organize content with expandable overview, structure, and files sections
- **Search Functionality**: Quickly find specific files with real-time search
- **Syntax Highlighting**: Code is displayed with appropriate language detection
- **File Statistics**: Individual file sizes and line counts
- **Modern UI**: Clean, professional interface with gradient styling

### 📊 JSON Format (Structured Data)
Generates `codetree.json` - machine-readable format for integration:
- Complete project metadata
- Structured file information with metadata
- Statistics in programmatic format
- Perfect for CI/CD pipelines and automated analysis

### 📝 Markdown Format
Generates `codetree.md` - documentation-friendly format:
- GitHub-compatible markdown
- Structured sections with proper headers
- Code blocks with syntax highlighting
- Ideal for project documentation and README files

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin my-feature`
5. Submit a pull request

## 📜 License

This project is licensed under the **[MIT License](https://github.com/exyreams/Codetree?tab=MIT-1-ov-file)** - see the LICENSE file for details.

---

*Made with ❤️ by [exyreams](https://github.com/exyreams)*