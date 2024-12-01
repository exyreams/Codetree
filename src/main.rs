use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

const EXCLUDED_DIRS: [&str; 21] = [
    ".idea",
    ".git",
    ".github",
    ".gitlab",
    ".next",
    ".vscode",
    ".target",
    ".zig-cache",
    "node_modules",
    "assets",
    "asset",
    "public",
    "bin",
    "build",
    "cache",
    "dist",
    "fonts",
    "obj",
    "out",
    "target",
    "vendor",
];

const EXCLUDED_FILES: [&str; 25] = [
    ".DS_Store",
    ".env",
    ".eslintrc.json",
    ".gitignore",
    ".npmignore",
    "Cargo.lock",
    "eslint.config.js",
    "favicon.ico",
    "globals.css",
    "next.config.mjs",
    "next-env.d.ts",
    "postcss.config.js",
    "postcss.config.mjs",
    "README.md",
    "package-lock.json",
    "pnpm-lock.yaml",
    "tailwind.config.js",
    "tailwind.config.ts",
    "tsconfig.app.json",
    "tsconfig.node.json",
    "tsconfig.json",
    "thumbs.db",
    "tsconfig.json",
    "vite.config.ts",
    "yarn.lock",
];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let start_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    let script_name = env::args().next().unwrap();
    let output_file_name = "codetree.txt";
    let output_file_path = start_dir.join(output_file_name);

    if output_file_path.exists() {
        fs::remove_file(&output_file_path)?;
    }

    let mut file_paths = Vec::new();
    let mut output = String::new();

    println!("Generating file tree for {}...", start_dir.display());
    output.push_str("Project File Tree:\n\n");
    get_file_tree_and_contents(
        &start_dir,
        0,
        &mut file_paths,
        &mut output,
        &script_name,
        output_file_name,
    )?;

    output.push_str("\nProject Codes:\n\n");

    for (i, file) in file_paths.iter().enumerate() {
        let progress = (i + 1) as f32 / file_paths.len() as f32 * 100.0;
        print!("\rProcessing Files: {}% Complete", progress as u32);
        io::stdout().flush()?;

        if file.file_name().unwrap_or_default().to_str() == Some(&script_name)
            || file.file_name().unwrap_or_default() == OsStr::new(output_file_name)
            || is_excluded_file(file)
        {
            continue;
        }

        output.push_str(&format!(
            "{}. {}\n",
            i + 1,
            file.strip_prefix(&start_dir).unwrap_or(file).display()
        ));

        if file.exists() {
            match fs::read_to_string(file) {
                Ok(content) => {
                    output.push_str("\n");
                    output.push_str(&content);
                    output.push_str("\n");
                }
                Err(_) => output.push_str(" (Unable to read file content)\n"),
            }
        } else {
            output.push_str(" (File not found)\n");
        }
        output.push('\n');
    }

    println!("\nWriting to file...");
    fs::write(&output_file_path, output)?;

    println!(
        "File tree and contents have been written to {}",
        output_file_path.display()
    );
    Ok(())
}

fn get_file_tree_and_contents(
    dir: &Path,
    depth: usize,
    file_paths: &mut Vec<PathBuf>,
    output: &mut String,
    script_name: &str,
    output_file_name: &str,
) -> io::Result<()> {
    let indent = "│   ".repeat(depth);
    let last_indent = if depth > 0 {
        format!("{}└── ", "│   ".repeat(depth - 1))
    } else {
        String::new()
    };

    let mut entries: Vec<_> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_excluded(e))
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|a| {
        (
            !a.file_type().is_dir(),
            a.file_name().to_string_lossy().to_string(),
        )
    });

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let file_name = entry.file_name().to_string_lossy();

        if file_name == script_name
            || file_name == output_file_name
            || is_excluded_file(entry.path())
        {
            continue;
        }

        if entry.file_type().is_dir() {
            output.push_str(&format!(
                "{}{}{}/\n",
                if is_last { &last_indent } else { &indent },
                if is_last { "└── " } else { "├── " },
                file_name
            ));
            get_file_tree_and_contents(
                entry.path(),
                depth + 1,
                file_paths,
                output,
                script_name,
                output_file_name,
            )?;
        } else {
            output.push_str(&format!(
                "{}{}{}\n",
                if is_last { &last_indent } else { &indent },
                if is_last { "└── " } else { "├── " },
                file_name
            ));
            file_paths.push(entry.path().to_path_buf());
        }
    }

    Ok(())
}

fn is_excluded(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && EXCLUDED_DIRS.contains(&entry.file_name().to_str().unwrap_or(""))
}

fn is_excluded_file(path: &Path) -> bool {
    EXCLUDED_FILES.contains(&path.file_name().unwrap_or_default().to_str().unwrap_or(""))
}
