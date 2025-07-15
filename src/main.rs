//! Code Tree Generator
//!
//! This program scans a directory and generates a file tree along with code statistics.
//! It creates a detailed report including:
//! - A visual directory tree structure
//! - File counts by type/extension
//! - Line counts (total, code, comments, blank)
//! - File size information
//! - Project type and framework detection
//! - All source code content with sensitive information protection

mod output;

use clap::{Arg, Command};
use output::{
    formats::TextGenerator, html::HtmlGenerator, json::JsonGenerator, markdown::MarkdownGenerator,
    FileInfo, OutputFormat, OutputGenerator, ProjectReport,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Formats a file size in bytes into a human-readable string
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Calculates the total size and file count of a directory recursively
fn calculate_directory_size(dir_path: &Path) -> io::Result<(u64, usize)> {
    let mut total_size = 0u64;
    let mut file_count = 0usize;

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            file_count += 1;
            if let Ok(size) = get_real_file_size(entry.path()) {
                total_size += size;
            }
        }
    }

    Ok((total_size, file_count))
}

// Platform-specific imports for real file size detection
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use winapi::um::fileapi::{GetCompressedFileSizeW, GetFileAttributesW, INVALID_FILE_SIZE, INVALID_FILE_ATTRIBUTES};
#[cfg(windows)]
use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Gets the real file size on disk (cross-platform)
/// This accounts for file system allocation, compression, sparse files, etc.
fn get_real_file_size(path: &Path) -> io::Result<u64> {
    #[cfg(windows)]
    {
        get_real_file_size_windows(path)
    }
    #[cfg(unix)]
    {
        get_real_file_size_unix(path)
    }
    #[cfg(not(any(windows, unix)))]
    {
        // Fallback to standard metadata for other platforms
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
}

#[cfg(windows)]
fn get_real_file_size_windows(path: &Path) -> io::Result<u64> {
    
    // Convert path to wide string for Windows API
    let wide_path: Vec<u16> = path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        // Check if it's a directory first
        let attributes = GetFileAttributesW(wide_path.as_ptr());
        if attributes != INVALID_FILE_ATTRIBUTES 
            && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0 {
            return Ok(0); // Directories have no size
        }
        
        // Get compressed file size (actual disk usage)
        let mut high_part: u32 = 0;
        let low_part = GetCompressedFileSizeW(wide_path.as_ptr(), &mut high_part);
        
        if low_part == INVALID_FILE_SIZE {
            // Fall back to regular file size if compressed size fails
            let metadata = fs::metadata(path)?;
            Ok(metadata.len())
        } else {
            // Combine high and low parts to get full 64-bit size
            let size = ((high_part as u64) << 32) | (low_part as u64);
            Ok(size)
        }
    }
}

#[cfg(unix)]
fn get_real_file_size_unix(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    
    // On Unix systems, use st_blocks * 512 to get actual disk usage
    // st_blocks is the number of 512-byte blocks allocated
    let blocks = metadata.blocks();
    let block_size = 512u64;
    
    // Calculate actual disk usage
    let disk_usage = blocks * block_size;
    
    // Return the smaller of logical size or disk usage
    // (sparse files can have less disk usage than logical size)
    let logical_size = metadata.len();
    Ok(std::cmp::min(disk_usage, logical_size))
}

/// Base directories to always exclude from the analysis
const BASE_EXCLUDED_DIRS: [&str; 10] = [
    ".idea", ".git", ".github", ".gitlab", ".vscode", ".venv", "cache", "fonts", "obj", "out",
];

/// Files to exclude from the analysis
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

/// Files that may contain sensitive information
const SENSITIVE_FILES: [&str; 15] = [
    ".env",
    ".env.local",
    ".env.development",
    ".env.production",
    ".env.test",
    "config.json",
    "secrets.json",
    "credentials.json",
    "aws-config.json",
    "firebase-config.json",
    "database.yml",
    "settings.py",
    "config.py",
    "wp-config.php",
    "application.properties",
];

/// Structure for tracking detected frameworks
struct FrameworkDetection {
    /// Map of detected frameworks and their versions
    frameworks: HashMap<String, String>,
}

impl FrameworkDetection {
    /// Creates a new empty FrameworkDetection
    fn new() -> Self {
        FrameworkDetection {
            frameworks: HashMap::new(),
        }
    }

    /// Detects frameworks in a JavaScript/Node.js project
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory
    /// * `package_json` - Contents of package.json file
    fn detect_js_frameworks(&mut self, _root_dir: &Path, package_json: &str) {
        // Helper function to extract version from dependency
        let extract_version = |json: &str, dep_name: &str| -> Option<String> {
            let dep_pattern = format!("\"{}\"\\s*:\\s*\"([^\"]+)\"", dep_name);
            match Regex::new(&dep_pattern) {
                Ok(re) => re.captures(json).map(|cap| cap[1].to_string()),
                Err(_) => {
                    // Fallback simple string-based approach if regex fails
                    let search_str = format!("\"{}\":", dep_name);
                    if let Some(idx) = json.find(&search_str) {
                        let start_idx = json[idx..].find("\"").unwrap_or(0) + idx + 1;
                        let end_idx = json[start_idx..].find("\"").unwrap_or(0) + start_idx;
                        if start_idx < end_idx {
                            return Some(json[start_idx..end_idx].to_string());
                        }
                    }
                    None
                }
            }
        };

        // Frontend frameworks
        if package_json.contains("\"react\"") {
            let version = extract_version(package_json, "react").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("React".to_string(), version);
        }

        if package_json.contains("\"vue\"") {
            let version = extract_version(package_json, "vue").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Vue.js".to_string(), version);
        }

        if package_json.contains("\"@angular/core\"") {
            let version =
                extract_version(package_json, "@angular/core").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Angular".to_string(), version);
        }

        if package_json.contains("\"next\"") {
            let version = extract_version(package_json, "next").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Next.js".to_string(), version);
        }

        if package_json.contains("\"three\"") {
            let version = extract_version(package_json, "three").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Three.js".to_string(), version);
        }

        if package_json.contains("\"svelte\"") {
            let version =
                extract_version(package_json, "svelte").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Svelte".to_string(), version);
        }

        // CSS frameworks and UI libraries
        if package_json.contains("\"tailwindcss\"") {
            let version =
                extract_version(package_json, "tailwindcss").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Tailwind CSS".to_string(), version);
        }

        if package_json.contains("\"@mui/material\"")
            || package_json.contains("\"@material-ui/core\"")
        {
            let version = extract_version(package_json, "@mui/material")
                .or_else(|| extract_version(package_json, "@material-ui/core"))
                .unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Material UI".to_string(), version);
        }

        if package_json.contains("\"bootstrap\"") {
            let version =
                extract_version(package_json, "bootstrap").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Bootstrap".to_string(), version);
        }

        if package_json.contains("\"@chakra-ui/react\"") {
            let version = extract_version(package_json, "@chakra-ui/react")
                .unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Chakra UI".to_string(), version);
        }

        // Backend frameworks
        if package_json.contains("\"express\"") {
            let version =
                extract_version(package_json, "express").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Express.js".to_string(), version);
        }

        if package_json.contains("\"@nestjs/core\"") {
            let version =
                extract_version(package_json, "@nestjs/core").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("NestJS".to_string(), version);
        }

        if package_json.contains("\"fastify\"") {
            let version =
                extract_version(package_json, "fastify").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Fastify".to_string(), version);
        }

        // State management
        if package_json.contains("\"redux\"") {
            let version = extract_version(package_json, "redux").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Redux".to_string(), version);
        }

        if package_json.contains("\"mobx\"") {
            let version = extract_version(package_json, "mobx").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("MobX".to_string(), version);
        }

        // Testing frameworks
        if package_json.contains("\"jest\"") {
            let version = extract_version(package_json, "jest").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Jest".to_string(), version);
        }

        if package_json.contains("\"cypress\"") {
            let version =
                extract_version(package_json, "cypress").unwrap_or_else(|| "?".to_string());
            self.frameworks.insert("Cypress".to_string(), version);
        }
    }

    /// Detects frameworks in a Python project
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory
    fn detect_python_frameworks(&mut self, root_dir: &Path) {
        // Check for requirements.txt
        let requirements_path = root_dir.join("requirements.txt");
        if requirements_path.exists() {
            if let Ok(content) = fs::read_to_string(&requirements_path) {
                // Django
                if content.contains("django") {
                    self.frameworks
                        .insert("Django".to_string(), "?".to_string());
                }

                // Flask
                if content.contains("flask") {
                    self.frameworks.insert("Flask".to_string(), "?".to_string());
                }

                // FastAPI
                if content.contains("fastapi") {
                    self.frameworks
                        .insert("FastAPI".to_string(), "?".to_string());
                }

                // SQLAlchemy
                if content.contains("sqlalchemy") {
                    self.frameworks
                        .insert("SQLAlchemy".to_string(), "?".to_string());
                }

                // Pytest
                if content.contains("pytest") {
                    self.frameworks
                        .insert("Pytest".to_string(), "?".to_string());
                }
            }
        }

        // Check for Django-specific files
        if root_dir.join("manage.py").exists()
            && (root_dir.join("settings.py").exists()
                || fs::read_dir(root_dir).map_or(false, |entries| {
                    entries
                        .filter_map(Result::ok)
                        .any(|e| e.path().join("settings.py").exists())
                }))
        {
            self.frameworks
                .insert("Django".to_string(), "?".to_string());
        }
    }

    /// Detects frameworks in a Ruby project
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory
    fn detect_ruby_frameworks(&mut self, root_dir: &Path) {
        // Ruby on Rails
        if root_dir.join("config").join("routes.rb").exists() {
            self.frameworks
                .insert("Ruby on Rails".to_string(), "?".to_string());
        }
    }

    /// Detects frameworks in a PHP project
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory
    fn detect_php_frameworks(&mut self, root_dir: &Path) {
        // Laravel
        if root_dir.join("artisan").exists() {
            self.frameworks
                .insert("Laravel".to_string(), "?".to_string());
        }

        // Symfony
        if root_dir.join("bin").join("console").exists()
            && root_dir.join("config").exists()
            && root_dir.join("src").join("Kernel.php").exists()
        {
            self.frameworks
                .insert("Symfony".to_string(), "?".to_string());
        }
    }

    /// Detects frameworks in a Java project
    ///
    /// # Arguments
    ///
    /// * `_root_dir` - Project root directory (unused but kept for API consistency)
    /// * `pom_xml` - Optional contents of pom.xml file
    fn detect_java_frameworks(&mut self, _root_dir: &Path, pom_xml: Option<&str>) {
        // Spring Boot from pom.xml
        if let Some(content) = pom_xml {
            if content.contains("spring-boot") {
                self.frameworks
                    .insert("Spring Boot".to_string(), "?".to_string());
            }

            if content.contains("hibernate") {
                self.frameworks
                    .insert("Hibernate".to_string(), "?".to_string());
            }
        }
    }

    /// Detects frameworks in a .NET project
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory
    fn detect_dotnet_frameworks(&mut self, root_dir: &Path) {
        // Look for .csproj files
        let entries = WalkDir::new(root_dir)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
                ext == "csproj"
            });

        for entry in entries {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if content.contains("Microsoft.AspNetCore") {
                    self.frameworks
                        .insert("ASP.NET Core".to_string(), "?".to_string());
                    break;
                }
            }
        }
    }

    /// Returns a formatted string of detected frameworks
    fn format_frameworks(&self) -> String {
        if self.frameworks.is_empty() {
            return "No specific frameworks detected\n".to_string();
        }

        let mut result = String::new();
        result.push_str("Detected Frameworks:\n");

        // Group frameworks by category
        let mut frontend = Vec::new();
        let mut backend = Vec::new();
        let mut testing = Vec::new();
        let mut other = Vec::new();

        for (name, version) in &self.frameworks {
            match name.as_str() {
                "React" | "Vue.js" | "Angular" | "Next.js" | "Three.js" | "Svelte"
                | "Tailwind CSS" | "Material UI" | "Bootstrap" | "Chakra UI" => {
                    frontend.push((name, version));
                }
                "Express.js" | "NestJS" | "Fastify" | "Django" | "Flask" | "FastAPI"
                | "Ruby on Rails" | "Laravel" | "Symfony" | "Spring Boot" | "ASP.NET Core" => {
                    backend.push((name, version));
                }
                "Jest" | "Cypress" | "Pytest" => {
                    testing.push((name, version));
                }
                _ => {
                    other.push((name, version));
                }
            }
        }

        // Helper function to format a group
        let format_group = |name: &str, items: &[(&String, &String)]| -> String {
            if items.is_empty() {
                return String::new();
            }

            let mut result = format!("  {} Frameworks:\n", name);
            for (framework, version) in items {
                result.push_str(&format!("    - {} (v{})\n", framework, version));
            }
            result
        };

        // Add each group to the result
        result.push_str(&format_group("Frontend", &frontend));
        result.push_str(&format_group("Backend", &backend));
        result.push_str(&format_group("Testing", &testing));
        result.push_str(&format_group("Other", &other));

        result
    }
}

/// Structure to track excluded files and directories with their sizes
#[derive(Debug)]
struct ExcludedStats {
    /// Total size of all excluded content
    total_excluded_size: u64,
    /// Excluded directories with their sizes and file counts
    excluded_directories: Vec<(String, u64, usize, String)>, // (path, size, file_count, reason)
    /// Excluded files with their sizes
    excluded_files: Vec<(String, u64, String)>, // (path, size, reason)
}

impl ExcludedStats {
    fn new() -> Self {
        ExcludedStats {
            total_excluded_size: 0,
            excluded_directories: Vec::new(),
            excluded_files: Vec::new(),
        }
    }

    /// Adds an excluded directory with its calculated size and file count
    fn add_excluded_directory(&mut self, path: &str, size: u64, file_count: usize, reason: &str) {
        self.total_excluded_size += size;
        self.excluded_directories.push((path.to_string(), size, file_count, reason.to_string()));
    }

    /// Adds an excluded file with its size
    fn add_excluded_file(&mut self, path: &str, size: u64, reason: &str) {
        self.total_excluded_size += size;
        self.excluded_files.push((path.to_string(), size, reason.to_string()));
    }

    /// Formats the exclusion statistics into a human-readable string
    fn format_excluded_stats(&self) -> String {
        let mut result = String::new();
        
        if self.total_excluded_size == 0 {
            return result;
        }

        result.push_str("\nExcluded Content Analysis:\n");
        result.push_str("==========================\n");
        result.push_str(&format!("Total excluded content size: {}\n\n", format_size(self.total_excluded_size)));

        // Top 10 largest excluded directories
        if !self.excluded_directories.is_empty() {
            result.push_str("Top 10 Largest Excluded Directories:\n");
            let mut sorted_dirs = self.excluded_directories.clone();
            sorted_dirs.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (i, (path, size, file_count, reason)) in sorted_dirs.iter().take(10).enumerate() {
                result.push_str(&format!(
                    "  {}. {} - {} ({} files) - Reason: {}\n",
                    i + 1,
                    path,
                    format_size(*size),
                    file_count,
                    reason
                ));
            }
            result.push('\n');
        }

        // Top 10 largest excluded files
        if !self.excluded_files.is_empty() {
            result.push_str("Top 10 Largest Excluded Files:\n");
            let mut sorted_files = self.excluded_files.clone();
            sorted_files.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (i, (path, size, reason)) in sorted_files.iter().take(10).enumerate() {
                result.push_str(&format!(
                    "  {}. {} - {} - Reason: {}\n",
                    i + 1,
                    path,
                    format_size(*size),
                    reason
                ));
            }
            result.push('\n');
        }

        result
    }
}

/// Structure to track project type and customized exclusions
struct ProjectDetector {
    /// Set of directories to exclude
    excluded_dirs: HashSet<String>,
    /// Detected project types
    project_types: HashSet<String>,
    /// Framework detector
    framework_detection: FrameworkDetection,
    /// Exclusion statistics tracker
    excluded_stats: ExcludedStats,
}

impl ProjectDetector {
    /// Creates a new ProjectDetector with base exclusions
    fn new() -> Self {
        let mut excluded_dirs = HashSet::new();
        for dir in BASE_EXCLUDED_DIRS.iter() {
            excluded_dirs.insert(dir.to_string());
        }

        ProjectDetector {
            excluded_dirs,
            project_types: HashSet::new(),
            framework_detection: FrameworkDetection::new(),
            excluded_stats: ExcludedStats::new(),
        }
    }

    /// Detects project types and updates exclusions based on root directory content
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Project root directory to analyze
    fn detect_project_types(&mut self, root_dir: &Path) -> io::Result<()> {
        // Check for Rust project
        if root_dir.join("Cargo.toml").exists() {
            self.project_types.insert("Rust".to_string());
            self.excluded_dirs.insert("target".to_string());
        }

        // Check for Node.js/JavaScript/React project
        let package_json_path = root_dir.join("package.json");
        if package_json_path.exists() {
            self.project_types.insert("JavaScript/Node.js".to_string());
            self.excluded_dirs.insert("node_modules".to_string());
            self.excluded_dirs.insert("dist".to_string());
            self.excluded_dirs.insert("build".to_string());

            // Detect JS frameworks from package.json
            if let Ok(package_json) = fs::read_to_string(&package_json_path) {
                self.framework_detection
                    .detect_js_frameworks(root_dir, &package_json);
            }
        }

        // Check for Python project
        if root_dir.join("setup.py").exists()
            || root_dir.join("requirements.txt").exists()
            || root_dir.join("pyproject.toml").exists()
        {
            self.project_types.insert("Python".to_string());
            self.excluded_dirs.insert("__pycache__".to_string());
            self.excluded_dirs.insert(".pytest_cache".to_string());
            self.excluded_dirs.insert("venv".to_string());
            self.excluded_dirs.insert("dist".to_string());
            self.excluded_dirs.insert("build".to_string());

            // Detect Python frameworks
            self.framework_detection.detect_python_frameworks(root_dir);
        }

        // Check for Java/Maven project
        let pom_xml_path = root_dir.join("pom.xml");
        if pom_xml_path.exists() {
            self.project_types.insert("Java/Maven".to_string());
            self.excluded_dirs.insert("target".to_string());

            // Detect Java frameworks
            if let Ok(pom_xml) = fs::read_to_string(&pom_xml_path) {
                self.framework_detection
                    .detect_java_frameworks(root_dir, Some(&pom_xml));
            }
        }

        // Check for Java/Gradle project
        if root_dir.join("build.gradle").exists() || root_dir.join("build.gradle.kts").exists() {
            self.project_types.insert("Java/Gradle".to_string());
            self.excluded_dirs.insert("build".to_string());
            self.excluded_dirs.insert(".gradle".to_string());

            // Detect Java frameworks without pom.xml
            self.framework_detection
                .detect_java_frameworks(root_dir, None);
        }

        // Check for .NET project
        if let Ok(entries) = fs::read_dir(root_dir) {
            let has_csproj = entries.filter_map(Result::ok).any(|e| {
                let path = e.path();
                let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
                ext == "csproj" || ext == "fsproj"
            });

            if has_csproj {
                self.project_types.insert(".NET".to_string());
                self.excluded_dirs.insert("bin".to_string());
                self.excluded_dirs.insert("obj".to_string());

                // Detect .NET frameworks
                self.framework_detection.detect_dotnet_frameworks(root_dir);
            }
        }

        // Check for Go project
        if root_dir.join("go.mod").exists() {
            self.project_types.insert("Go".to_string());
            self.excluded_dirs.insert("vendor".to_string());
        }

        // Check for Ruby project and frameworks
        if root_dir.join("Gemfile").exists() {
            self.project_types.insert("Ruby".to_string());
            self.framework_detection.detect_ruby_frameworks(root_dir);
        }

        // Check for PHP project and frameworks
        if root_dir.join("composer.json").exists() {
            self.project_types.insert("PHP".to_string());
            self.excluded_dirs.insert("vendor".to_string());
            self.framework_detection.detect_php_frameworks(root_dir);
        }

        // Add commonly excluded directories for all projects
        self.excluded_dirs.insert("assets".to_string());
        self.excluded_dirs.insert("asset".to_string());
        self.excluded_dirs.insert("public".to_string());
        self.excluded_dirs.insert("bin".to_string());

        Ok(())
    }

    /// Formats detected project information
    ///
    /// # Returns
    ///
    /// * `String` - Formatted project type information
    fn format_project_info(&self) -> String {
        let mut result = String::new();

        if !self.project_types.is_empty() {
            result.push_str("Detected Project Types: ");
            let project_types: Vec<_> = self.project_types.iter().collect();
            for (i, project_type) in project_types.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(project_type);
            }
            result.push_str("\n");

            result.push_str("Auto-excluded build directories: ");
            let dirs: Vec<_> = self
                .excluded_dirs
                .iter()
                .filter(|dir| !BASE_EXCLUDED_DIRS.contains(&dir.as_str()))
                .collect();

            for (i, dir) in dirs.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(dir);
            }
            result.push_str("\n\n");
        } else {
            result.push_str("No specific project type detected\n\n");
        }

        // Add framework information
        result.push_str(&self.framework_detection.format_frameworks());

        // Add exclusion statistics
        result.push_str(&self.excluded_stats.format_excluded_stats());

        result
    }

    /// Checks if a directory should be excluded
    ///
    /// # Arguments
    ///
    /// * `dir_name` - Name of the directory to check
    ///
    /// # Returns
    ///
    /// * `bool` - True if the directory should be excluded
    fn is_excluded_dir(&self, dir_name: &str) -> bool {
        self.excluded_dirs.contains(dir_name)
    }
}

/// Structure to track and calculate project statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStats {
    /// Total number of lines across all files
    total_lines: usize,
    /// Number of code lines (non-blank, non-comment)
    code_lines: usize,
    /// Number of blank lines
    blank_lines: usize,
    /// Number of comment lines
    comment_lines: usize,
    /// Total number of files analyzed
    total_files: usize,
    /// Count of files by extension
    files_by_extension: HashMap<String, usize>,
    /// Count of lines by extension
    lines_by_extension: HashMap<String, usize>,
    /// Total size of all files in bytes (actual file system size)
    total_size_bytes: u64,
    /// Total content size in bytes (UTF-8 string content size)
    total_content_size_bytes: u64,
    /// Size breakdown by file extension
    size_by_extension: HashMap<String, u64>,
    /// Count of potentially sensitive files
    sensitive_files_count: usize,
    /// Largest files (path, size) - top 10
    largest_files: Vec<(String, u64)>,
    /// Average file size
    average_file_size: u64,
}

impl ProjectStats {
    /// Creates a new empty ProjectStats instance
    fn new() -> Self {
        ProjectStats {
            total_lines: 0,
            code_lines: 0,
            blank_lines: 0,
            comment_lines: 0,
            total_files: 0,
            files_by_extension: HashMap::new(),
            lines_by_extension: HashMap::new(),
            total_size_bytes: 0,
            total_content_size_bytes: 0,
            size_by_extension: HashMap::new(),
            sensitive_files_count: 0,
            largest_files: Vec::new(),
            average_file_size: 0,
        }
    }

    /// Analyzes a file and updates statistics
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to analyze
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Success or error result
    fn add_file(&mut self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            return Ok(());
        }

        self.total_files += 1;

        // Check if file might contain sensitive information
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let is_sensitive = SENSITIVE_FILES.iter().any(|f| file_name.ends_with(f));

        if is_sensitive {
            self.sensitive_files_count += 1;
        }

        // Get file extension and update counts
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            let ext = ext.to_lowercase();
            *self.files_by_extension.entry(ext.clone()).or_insert(0) += 1;
        }

        // Get accurate file size information using cross-platform real size detection
        let mut file_size = 0u64;
        
        // Use our enhanced file size detection that accounts for compression, allocation, etc.
        if let Ok(real_size) = get_real_file_size(path) {
            file_size = real_size;
            self.total_size_bytes += file_size;
        }

        // Get file extension for size tracking
        let extension = path.extension()
            .and_then(OsStr::to_str)
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "no_extension".to_string());

        // Track size by extension
        *self.size_by_extension.entry(extension.clone()).or_insert(0) += file_size;

        // Count lines and analyze content
        if let Ok(content) = fs::read_to_string(path) {
            let lines: Vec<_> = content.lines().collect();
            let line_count = lines.len();

            // Calculate content size (UTF-8 bytes)
            let content_size = content.len() as u64;
            self.total_content_size_bytes += content_size;

            self.total_lines += line_count;

            // Track lines by extension
            *self.lines_by_extension.entry(extension).or_insert(0) += line_count;

            // Count blank lines and comments
            let mut blank = 0;
            let mut comments = 0;

            for line in &lines {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    blank += 1;
                } else if is_likely_comment(trimmed, path) {
                    comments += 1;
                }
            }

            self.blank_lines += blank;
            self.comment_lines += comments;
            self.code_lines += line_count - blank - comments;
        }

        // Track largest files (keep top 10)
        let file_path_str = path.display().to_string();
        self.largest_files.push((file_path_str, file_size));
        self.largest_files.sort_by(|a, b| b.1.cmp(&a.1));
        if self.largest_files.len() > 10 {
            self.largest_files.truncate(10);
        }

        Ok(())
    }

    /// Finalizes statistics calculations (call after all files are processed)
    fn finalize(&mut self) {
        // Calculate average file size
        if self.total_files > 0 {
            self.average_file_size = self.total_size_bytes / self.total_files as u64;
        }
    }

    /// Formats the statistics into a human-readable string
    ///
    /// # Returns
    ///
    /// * `String` - Formatted statistics report
    fn format_stats(&self) -> String {
        let mut result = String::new();

        result.push_str("\nProject Statistics:\n");
        result.push_str("==================\n");
        result.push_str(&format!("Total Files: {}\n", self.total_files));
        result.push_str(&format!("Total Lines of Code: {}\n", self.total_lines));
        result.push_str(&format!(
            "  - Code Lines: {} ({:.1}%)\n",
            self.code_lines,
            if self.total_lines > 0 {
                self.code_lines as f64 / self.total_lines as f64 * 100.0
            } else {
                0.0
            }
        ));
        result.push_str(&format!(
            "  - Comment Lines: {} ({:.1}%)\n",
            self.comment_lines,
            if self.total_lines > 0 {
                self.comment_lines as f64 / self.total_lines as f64 * 100.0
            } else {
                0.0
            }
        ));
        result.push_str(&format!(
            "  - Blank Lines: {} ({:.1}%)\n",
            self.blank_lines,
            if self.total_lines > 0 {
                self.blank_lines as f64 / self.total_lines as f64 * 100.0
            } else {
                0.0
            }
        ));

        // Enhanced file size information
        let total_size_str = format_size(self.total_size_bytes);
        let content_size_str = format_size(self.total_content_size_bytes);
        let average_size_str = format_size(self.average_file_size);
        
        result.push_str(&format!("Total File System Size: {}\n", total_size_str));
        result.push_str(&format!("Total Content Size: {}\n", content_size_str));
        result.push_str(&format!("Average File Size: {}\n", average_size_str));
        
        // Show compression ratio if there's a difference
        if self.total_size_bytes > 0 && self.total_content_size_bytes > 0 {
            let ratio = (self.total_content_size_bytes as f64 / self.total_size_bytes as f64) * 100.0;
            result.push_str(&format!("Content to File Size Ratio: {:.1}%\n", ratio));
        }

        if self.sensitive_files_count > 0 {
            result.push_str(&format!(
                "\nDetected {} potentially sensitive file(s) that have been protected.\n",
                self.sensitive_files_count
            ));
        }

        // Enhanced files by extension with size information
        result.push_str("\nFiles by Type:\n");
        let mut extensions: Vec<_> = self.files_by_extension.iter().collect();
        extensions.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count (descending)

        for (ext, count) in extensions {
            let lines = self.lines_by_extension.get(ext).unwrap_or(&0);
            let size = self.size_by_extension.get(ext).unwrap_or(&0);
            let size_str = format_size(*size);
            result.push_str(&format!("  .{}: {} files, {} lines, {}\n", ext, count, lines, size_str));
        }

        // Show largest files
        if !self.largest_files.is_empty() {
            result.push_str("\nLargest Files:\n");
            for (i, (path, size)) in self.largest_files.iter().enumerate() {
                if i >= 5 { break; } // Show top 5 largest files
                let size_str = format_size(*size);
                // Show only the filename, not the full path for readability
                let filename = std::path::Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                result.push_str(&format!("  {}: {}\n", filename, size_str));
            }
        }

        result
    }
}



/// Determines if a line is likely a comment based on file extension
///
/// # Arguments
///
/// * `line` - The line to check
/// * `path` - Path to the file, used to determine file type
///
/// # Returns
///
/// * `bool` - True if the line is likely a comment
fn is_likely_comment(line: &str, path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
        match ext.to_lowercase().as_str() {
            "rs" | "c" | "cpp" | "h" | "hpp" | "js" | "jsx" | "ts" | "tsx" | "java" | "cs"
            | "go" | "swift" => {
                line.starts_with("//")
                    || line.starts_with("/*")
                    || line.starts_with("*")
                    || line.starts_with("*/")
            }
            "py" | "rb" | "sh" | "bash" | "yml" | "yaml" => line.starts_with("#"),
            "html" | "xml" | "svg" => line.starts_with("<!--") || line.contains("-->"),
            "css" | "scss" | "sass" => {
                line.starts_with("/*")
                    || line.starts_with("*/")
                    || line.starts_with("*")
                    || line.starts_with("//")
            }
            _ => false,
        }
    } else {
        false
    }
}

/// Checks if a file might contain sensitive information
///
/// # Arguments
///
/// * `path` - Path to the file to check
/// * `content` - Optional content of the file
///
/// # Returns
///
/// * `bool` - True if the file might contain sensitive information
fn might_contain_sensitive_info(path: &Path, _content: Option<&str>) -> bool {
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Check if filename matches known sensitive files
    if SENSITIVE_FILES.iter().any(|f| file_name.ends_with(f)) {
        return true;
    }

    false
}

/// Main entry point of the application
fn main() -> io::Result<()> {
    let matches = Command::new("codetree")
        .version("2.1.0")
        .author("exyreams")
        .about("Generate comprehensive project analysis with multiple output formats")
        .arg(
            Arg::new("directory")
                .help("Directory to analyze")
                .value_name("DIR")
                .index(1),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format: text, json, markdown, html")
                .default_value("text"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file name (without extension)"),
        )
        .get_matches();

    let start_dir = if let Some(dir) = matches.get_one::<String>("directory") {
        PathBuf::from(dir)
    } else {
        env::current_dir()?
    };

    let format: OutputFormat = matches
        .get_one::<String>("format")
        .unwrap()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let output_name = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("codetree");

    // Initialize project detector and detect project types
    let mut project_detector = ProjectDetector::new();
    project_detector.detect_project_types(&start_dir)?;

    println!("{}", project_detector.format_project_info());

    let mut file_paths = Vec::new();
    let mut file_tree_output = String::new();
    let mut stats = ProjectStats::new();

    println!("Generating file tree for {}...", start_dir.display());

    get_file_tree_and_contents(
        &start_dir,
        0,
        &mut file_paths,
        &mut file_tree_output,
        &env::args().next().unwrap(),
        &format!("{}.{}", output_name, get_generator(&format).file_extension()),
        &mut stats,
        &mut project_detector,
    )?;

    // Collect file information
    let mut files = Vec::new();
    for (i, file_path) in file_paths.iter().enumerate() {
        let progress = (i + 1) as f32 / file_paths.len() as f32 * 100.0;
        print!("\rProcessing Files: {}% Complete", progress as u32);
        io::stdout().flush()?;

        let relative_path = file_path
            .strip_prefix(&start_dir)
            .unwrap_or(file_path)
            .display()
            .to_string();

        let is_sensitive = might_contain_sensitive_info(file_path, None);
        let mut content = None;
        let mut size_bytes = 0;
        let mut line_count = 0;

        if file_path.exists() {
            if let Ok(metadata) = fs::metadata(file_path) {
                size_bytes = metadata.len();
            }

            if !is_sensitive {
                if let Ok(file_content) = fs::read_to_string(file_path) {
                    line_count = file_content.lines().count();
                    content = Some(file_content);
                }
            }
        }

        files.push(FileInfo {
            path: file_path.display().to_string(),
            relative_path,
            content,
            is_sensitive,
            size_bytes,
            line_count,
        });
    }

    // Create project report
    let report = ProjectReport {
        project_info: project_detector.format_project_info(),
        file_tree: file_tree_output,
        statistics: stats,
        files,
        generated_at: chrono::Utc::now(),
    };

    // Generate output using the appropriate generator
    let generator = get_generator(&format);
    let output_content = generator
        .generate(&report)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;

    // Write output file
    let output_file_path = start_dir.join(format!("{}.{}", output_name, generator.file_extension()));
    
    if output_file_path.exists() {
        fs::remove_file(&output_file_path)?;
    }

    println!("\nWriting to file...");
    fs::write(&output_file_path, output_content)?;

    // Display summary statistics in terminal
    println!("{}", report.statistics.format_stats());
    println!(
        "Analysis complete! Report written to {}",
        output_file_path.display()
    );

    Ok(())
}

/// Returns the appropriate output generator for the given format
fn get_generator(format: &OutputFormat) -> Box<dyn OutputGenerator> {
    match format {
        OutputFormat::Text => Box::new(TextGenerator),
        OutputFormat::Json => Box::new(JsonGenerator),
        OutputFormat::Markdown => Box::new(MarkdownGenerator),
        OutputFormat::Html => Box::new(HtmlGenerator),
    }
}

/// Recursively generates a file tree and collects file paths
///
/// # Arguments
///
/// * `dir` - The directory to process
/// * `depth` - Current recursion depth for indentation
/// * `file_paths` - Vector to collect file paths
/// * `output` - String to append the tree representation
/// * `script_name` - Name of the script file to exclude
/// * `output_file_name` - Name of the output file to exclude
/// * `stats` - Statistics tracker to update
/// * `project_detector` - Project type detector with exclusion rules
///
/// # Returns
///
/// * `io::Result<()>` - Success or error result
fn get_file_tree_and_contents(
    dir: &Path,
    depth: usize,
    file_paths: &mut Vec<PathBuf>,
    output: &mut String,
    script_name: &str,
    output_file_name: &str,
    stats: &mut ProjectStats,
    project_detector: &mut ProjectDetector,
) -> io::Result<()> {
    let indent = "│   ".repeat(depth);
    let last_indent = if depth > 0 {
        format!("{}└── ", "│   ".repeat(depth - 1))
    } else {
        String::new()
    };

    // First collect all entries without filtering to track excluded ones
    let all_entries: Vec<_> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    // Separate included and excluded entries
    let mut entries = Vec::new();
    for entry in all_entries {
        if is_excluded(&entry, project_detector) {
            // Track excluded directory
            if entry.file_type().is_dir() {
                let dir_name = entry.file_name().to_str().unwrap_or("");
                let reason = get_exclusion_reason(dir_name, project_detector);
                if let Ok((size, file_count)) = calculate_directory_size(entry.path()) {
                    project_detector.excluded_stats.add_excluded_directory(
                        &entry.path().display().to_string(),
                        size,
                        file_count,
                        &reason,
                    );
                }
            }
        } else if is_excluded_file(entry.path()) {
            // Track excluded file
            let file_name = entry.file_name().to_str().unwrap_or("");
            let reason = get_file_exclusion_reason(file_name);
            if let Ok(size) = get_real_file_size(entry.path()) {
                project_detector.excluded_stats.add_excluded_file(
                    &entry.path().display().to_string(),
                    size,
                    &reason,
                );
            }
        } else {
            entries.push(entry);
        }
    }

    entries.sort_by_key(|a| {
        (
            !a.file_type().is_dir(),
            a.file_name().to_string_lossy().to_string(),
        )
    });

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let file_name = entry.file_name().to_string_lossy();

        if file_name == script_name || file_name == output_file_name {
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
                stats,
                project_detector,
            )?;
        } else {
            output.push_str(&format!(
                "{}{}{}\n",
                if is_last { &last_indent } else { &indent },
                if is_last { "└── " } else { "├── " },
                file_name
            ));
            file_paths.push(entry.path().to_path_buf());

            // Add file to statistics
            stats.add_file(entry.path())?;
        }
    }

    Ok(())
}

/// Checks if a directory entry should be excluded based on project detector
///
/// # Arguments
///
/// * `entry` - The directory entry to check
/// * `project_detector` - Project detector with exclusion rules
///
/// # Returns
///
/// * `bool` - True if the entry should be excluded
fn is_excluded(entry: &DirEntry, project_detector: &ProjectDetector) -> bool {
    if entry.file_type().is_dir() {
        let dir_name = entry.file_name().to_str().unwrap_or("");
        project_detector.is_excluded_dir(dir_name)
    } else {
        false
    }
}

/// Checks if a file should be excluded
///
/// # Arguments
///
/// * `path` - Path to the file to check
///
/// # Returns
///
/// * `bool` - True if the file should be excluded
fn is_excluded_file(path: &Path) -> bool {
    EXCLUDED_FILES.contains(&path.file_name().unwrap_or_default().to_str().unwrap_or(""))
}
/// Gets the exclusion reason for a directory
fn get_exclusion_reason(dir_name: &str, project_detector: &ProjectDetector) -> String {
    if BASE_EXCLUDED_DIRS.contains(&dir_name) {
        "Base excluded directory".to_string()
    } else if project_detector.excluded_dirs.contains(dir_name) {
        match dir_name {
            "target" => "Rust/Java build directory".to_string(),
            "node_modules" => "Node.js dependencies".to_string(),
            "dist" | "build" => "Build output directory".to_string(),
            "__pycache__" => "Python cache directory".to_string(),
            ".pytest_cache" => "Pytest cache directory".to_string(),
            "venv" => "Python virtual environment".to_string(),
            ".gradle" => "Gradle cache directory".to_string(),
            "bin" | "obj" => ".NET build directory".to_string(),
            "vendor" => "Dependencies directory".to_string(),
            "assets" | "asset" | "public" => "Static assets directory".to_string(),
            _ => "Project-specific excluded directory".to_string(),
        }
    } else {
        "Unknown exclusion reason".to_string()
    }
}

/// Gets the exclusion reason for a file
fn get_file_exclusion_reason(file_name: &str) -> String {
    match file_name {
        ".DS_Store" => "macOS system file".to_string(),
        ".env" | ".env.local" | ".env.development" | ".env.production" | ".env.test" => "Environment configuration file".to_string(),
        ".eslintrc.json" | "eslint.config.js" => "ESLint configuration".to_string(),
        ".gitignore" | ".npmignore" => "Version control ignore file".to_string(),
        "Cargo.lock" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" => "Dependency lock file".to_string(),
        "favicon.ico" => "Website icon file".to_string(),
        "globals.css" => "Global CSS file".to_string(),
        "next.config.mjs" | "next-env.d.ts" => "Next.js configuration".to_string(),
        "postcss.config.js" | "postcss.config.mjs" => "PostCSS configuration".to_string(),
        "README.md" => "Documentation file".to_string(),
        "tailwind.config.js" | "tailwind.config.ts" => "Tailwind CSS configuration".to_string(),
        "tsconfig.app.json" | "tsconfig.node.json" | "tsconfig.json" => "TypeScript configuration".to_string(),
        "thumbs.db" => "Windows thumbnail cache".to_string(),
        "vite.config.ts" => "Vite configuration".to_string(),
        _ => "Configuration/system file".to_string(),
    }
}