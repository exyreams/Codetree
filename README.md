
# Codetree

CodeTree is a Rust-based command-line tool that generates a comprehensive overview of a project's file structure and contents. It's designed to help developers quickly understand and document the layout and code of their projects.

## Why this tool?
Tired of the tedious task of manually creating file structures and populating them with code snippets when sharing your projects with LLMs or collaborators?  This tool automates that process, saving you valuable time and effort.

Imagine needing to provide your entire project's codebase to an LLM for analysis or to a colleague for review.  Manually copying and pasting each file's contents into a single text file is cumbersome and error-prone.  This tool eliminates that friction.

This Rust application generates a representative file tree mirroring the structure of a typical full-stack application (as shown in the example above) and compiles all the code into a single, easy-to-share `.txt` file. This makes sharing your code with LLMs, for tasks like code review, debugging, or generation of new code remarkably streamlined. No more tedious manual copying and pasting; instead, you'll have all the contextual information available at your fingertips, for immediate access to the source code,  without needing to reconstruct the directory structures.

**Stop wasting time on repetitive tasks and start focusing on what truly matters: building your application!  And it works on every OS.**


## Installation

1. **Ensure you have Rust installed on your system.** If not, install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. **For Windows Users (Easiest Method):**

    Download the latest Windows release directly from the [Releases page](https://github.com/exyreams/Codetree/releases). Download the `.exe` file and run it.


3. **For All Other Users (Compile from Source):**

   a. Clone this repository:
   ```bash
   git clone https://github.com/exyreams/Codetree.git
   cd Codetree
   ```

   b. Build the project:
   ```bash
   cargo build --release
   ```

4. **Run the project:**

   ```bash
   cargo run <PATH>
   ```


## Usage

Run CodeTree using one of the following commands:

1. To analyze the current directory:
   ```bash
   cargo run
   ```

2. To analyze a specific directory:
   - On Unix-like systems (Linux, macOS):
     ```bash
     cargo run /path/to/your/directory
     ```
        eg:  `cargo run /home/Desktop/Codetree`
   - On Windows:
     ```bash
     cargo run -- C:\path\to\your\directory
     ```
     eg:  `cargo run D:\Projects\Codetree`

The tool will generate an `codetree.txt` file in the analyzed directory, containing the file tree and the contents of each file.

## Output

The `codetree.txt` file will contain:

1. A visual representation of the project's file structure
2. The contents of each file in the project, excluding the script itself and the output file

## Customization

- You can modify the `EXCLUDED_DIRS` constant in the source code to adjust which directories are excluded from the analysis.  If you add or remove directories, ensure you update the `[&str; 19]` value accordingly; the number of items in the array must match.
- You can modify the `EXCLUDED_FILES` constant in the source code to adjust which files are excluded from the analysis.  If you add or remove files, ensure you update the `[&str; 8]` value accordingly; the number of items in the array must match.