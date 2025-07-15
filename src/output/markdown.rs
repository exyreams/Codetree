use super::{OutputGenerator, ProjectReport};

pub struct MarkdownGenerator;

impl OutputGenerator for MarkdownGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        
        // Header with badges and metadata
        output.push_str("# 🌳 Codetree Project Analysis\n\n");
        
        // Add badges
        output.push_str(&format!(
            "![Files](https://img.shields.io/badge/Files-{}-blue) \
            ![Lines](https://img.shields.io/badge/Lines-{}-green) \
            ![Size](https://img.shields.io/badge/Size-{}-orange) \
            ![Generated](https://img.shields.io/badge/Generated-{}-lightgrey)\n\n",
            report.statistics.total_files,
            report.statistics.total_lines,
            format_size(report.statistics.total_size_bytes).replace(" ", "%20"),
            report.generated_at.format("%Y--%m--%d")
        ));
        
        // Table of Contents
        output.push_str("## 📋 Table of Contents\n\n");
        output.push_str("- [📊 Project Overview](#-project-overview)\n");
        output.push_str("- [📈 Statistics Summary](#-statistics-summary)\n");
        if !report.statistics.files_by_extension.is_empty() {
            output.push_str("- [📁 File Type Analysis](#-file-type-analysis)\n");
        }
        output.push_str("- [🗂️ Project Structure](#️-project-structure)\n");
        output.push_str("- [📄 File Contents](#-file-contents)\n");
        output.push_str("\n---\n\n");
        
        // Project Overview
        output.push_str("## 📊 Project Overview\n\n");
        output.push_str(&format!("> **Analysis Generated:** {}\n\n", report.generated_at.format("%Y-%m-%d %H:%M:%S UTC")));
        
        if !report.project_info.trim().is_empty() {
            output.push_str("### Project Information\n\n");
            for line in report.project_info.lines() {
                if !line.trim().is_empty() {
                    output.push_str(&format!("{}\n", line));
                }
            }
            output.push_str("\n");
        }
        
        // Statistics Summary with visual elements
        output.push_str("## 📈 Statistics Summary\n\n");
        
        // Create a statistics table
        output.push_str("| Metric | Count | Percentage |\n");
        output.push_str("|--------|-------|------------|\n");
        
        let code_pct = if report.statistics.total_lines > 0 {
            report.statistics.code_lines as f64 / report.statistics.total_lines as f64 * 100.0
        } else { 0.0 };
        
        let comment_pct = if report.statistics.total_lines > 0 {
            report.statistics.comment_lines as f64 / report.statistics.total_lines as f64 * 100.0
        } else { 0.0 };
        
        let blank_pct = if report.statistics.total_lines > 0 {
            report.statistics.blank_lines as f64 / report.statistics.total_lines as f64 * 100.0
        } else { 0.0 };
        
        output.push_str(&format!("| 📁 **Total Files** | `{}` | - |\n", report.statistics.total_files));
        output.push_str(&format!("| 📏 **Total Lines** | `{}` | 100.0% |\n", report.statistics.total_lines));
        output.push_str(&format!("| 💻 **Code Lines** | `{}` | {:.1}% |\n", report.statistics.code_lines, code_pct));
        output.push_str(&format!("| 💬 **Comment Lines** | `{}` | {:.1}% |\n", report.statistics.comment_lines, comment_pct));
        output.push_str(&format!("| ⬜ **Blank Lines** | `{}` | {:.1}% |\n", report.statistics.blank_lines, blank_pct));
        output.push_str(&format!("| 💾 **Total Size** | `{}` | - |\n\n", format_size(report.statistics.total_size_bytes)));
        
        // Add progress bars using Unicode blocks
        output.push_str("### Code Composition\n\n");
        output.push_str(&format!("**Code Lines:** {:.1}%\n", code_pct));
        output.push_str(&format!("`{}`\n\n", create_progress_bar(code_pct, 30)));
        
        output.push_str(&format!("**Comments:** {:.1}%\n", comment_pct));
        output.push_str(&format!("`{}`\n\n", create_progress_bar(comment_pct, 30)));
        
        output.push_str(&format!("**Blank Lines:** {:.1}%\n", blank_pct));
        output.push_str(&format!("`{}`\n\n", create_progress_bar(blank_pct, 30)));
        
        // Files by extension with enhanced formatting
        if !report.statistics.files_by_extension.is_empty() {
            output.push_str("## 📁 File Type Analysis\n\n");
            
            let mut extensions: Vec<_> = report.statistics.files_by_extension.iter().collect();
            extensions.sort_by(|a, b| b.1.cmp(a.1));
            
            // Create a detailed table
            output.push_str("| Extension | Files | Lines | Percentage |\n");
            output.push_str("|-----------|-------|-------|------------|\n");
            
            for (ext, count) in &extensions {
                let lines = report.statistics.lines_by_extension.get(*ext).unwrap_or(&0);
                let file_pct = if report.statistics.total_files > 0 {
                    **count as f64 / report.statistics.total_files as f64 * 100.0
                } else { 0.0 };
                
                let lang_icon = get_language_icon(ext);
                output.push_str(&format!(
                    "| {} `.{}` | `{}` | `{}` | {:.1}% |\n",
                    lang_icon, ext, count, lines, file_pct
                ));
            }
            output.push_str("\n");
            
            // Add a visual breakdown for top file types
            output.push_str("### Top File Types\n\n");
            for (ext, count) in extensions.iter().take(5) {
                let lines = report.statistics.lines_by_extension.get(*ext).unwrap_or(&0);
                let file_pct = if report.statistics.total_files > 0 {
                    **count as f64 / report.statistics.total_files as f64 * 100.0
                } else { 0.0 };
                
                output.push_str(&format!("**{}** `.{}`\n", get_language_icon(ext), ext));
                output.push_str(&format!("- Files: {} ({:.1}%)\n", count, file_pct));
                output.push_str(&format!("- Lines: {}\n", lines));
                output.push_str(&format!("- Progress: `{}`\n\n", create_progress_bar(file_pct, 20)));
            }
        }
        
        // File tree with better formatting
        output.push_str("## 🗂️ Project Structure\n\n");
        output.push_str("<details>\n");
        output.push_str("<summary>Click to expand project structure</summary>\n\n");
        output.push_str("```\n");
        output.push_str(&report.file_tree);
        output.push_str("```\n\n");
        output.push_str("</details>\n\n");
        
        // File contents with enhanced organization
        output.push_str("## 📄 File Contents\n\n");
        
        // Group files by directory for better organization
        let mut files_by_dir: std::collections::BTreeMap<String, Vec<&crate::FileInfo>> = std::collections::BTreeMap::new();
        
        for file in &report.files {
            let dir = if let Some(pos) = file.relative_path.rfind('/') {
                file.relative_path[..pos].to_string()
            } else {
                ".".to_string()
            };
            files_by_dir.entry(dir).or_insert_with(Vec::new).push(file);
        }
        
        for (dir, files) in files_by_dir {
            if files.len() > 1 {
                output.push_str(&format!("### 📂 Directory: `{}`\n\n", dir));
            }
            
            for file in files {
                let file_icon = get_file_icon(&file.relative_path);
                output.push_str(&format!("#### {} `{}`\n\n", file_icon, file.relative_path));
                
                // Add file metadata
                output.push_str(&format!(
                    "> **Size:** {} | **Lines:** {} | **Language:** {}\n\n",
                    format_size(file.size_bytes),
                    file.line_count,
                    detect_language(&file.relative_path)
                ));
                
                if file.is_sensitive {
                    output.push_str("> ⚠️ **SENSITIVE FILE** - Content Protected\n\n");
                    output.push_str("```\n🔒 [SENSITIVE FILE - Content Protected]\n```\n\n");
                } else if let Some(content) = &file.content {
                    let lang = detect_language(&file.relative_path);
                    
                    // Add collapsible section for large files
                    if file.line_count > 50 {
                        output.push_str("<details>\n");
                        output.push_str(&format!("<summary>View file content ({} lines)</summary>\n\n", file.line_count));
                    }
                    
                    output.push_str(&format!("```{}\n", lang));
                    output.push_str(content);
                    if !content.ends_with('\n') {
                        output.push_str("\n");
                    }
                    output.push_str("```\n\n");
                    
                    if file.line_count > 50 {
                        output.push_str("</details>\n\n");
                    }
                } else {
                    output.push_str("```\n⚠️ [Unable to read file content]\n```\n\n");
                }
            }
        }
        
        // Footer
        output.push_str("---\n\n");
        output.push_str("*Generated by [Codetree](https://github.com/your-repo/codetree) - A powerful project analysis tool*\n");
        
        Ok(output)
    }
    
    fn file_extension(&self) -> &'static str {
        "md"
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} bytes", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn detect_language(file_path: &str) -> &str {
    if let Some(ext) = file_path.split('.').last() {
        match ext.to_lowercase().as_str() {
            "rs" => "rust",
            "js" | "jsx" => "javascript",
            "ts" | "tsx" => "typescript",
            "py" => "python",
            "java" => "java",
            "c" => "c",
            "cpp" | "cc" | "cxx" => "cpp",
            "h" | "hpp" => "c",
            "cs" => "csharp",
            "go" => "go",
            "rb" => "ruby",
            "php" => "php",
            "html" => "html",
            "css" => "css",
            "scss" | "sass" => "scss",
            "json" => "json",
            "xml" => "xml",
            "yml" | "yaml" => "yaml",
            "toml" => "toml",
            "md" => "markdown",
            "sh" | "bash" => "bash",
            "sql" => "sql",
            _ => "",
        }
    } else {
        ""
    }
}

fn create_progress_bar(percentage: f64, width: usize) -> String {
    let filled = (percentage / 100.0 * width as f64) as usize;
    let empty = width - filled;
    
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    
    format!("{} {:.1}%", bar, percentage)
}

fn get_language_icon(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "rs" => "🦀",
        "js" | "jsx" => "🟨",
        "ts" | "tsx" => "🔷",
        "py" => "🐍",
        "java" => "☕",
        "c" => "🔧",
        "cpp" | "cc" | "cxx" => "⚙️",
        "h" | "hpp" => "📋",
        "cs" => "🔷",
        "go" => "🐹",
        "rb" => "💎",
        "php" => "🐘",
        "html" => "🌐",
        "css" => "🎨",
        "scss" | "sass" => "💅",
        "json" => "📋",
        "xml" => "📄",
        "yml" | "yaml" => "⚙️",
        "toml" => "⚙️",
        "md" => "📝",
        "sh" | "bash" => "🐚",
        "sql" => "🗃️",
        "dockerfile" => "🐳",
        "gitignore" => "🚫",
        "lock" => "🔒",
        _ => "📄",
    }
}

fn get_file_icon(file_path: &str) -> &str {
    let filename = file_path.split('/').last().unwrap_or(file_path);
    
    // Special files
    match filename.to_lowercase().as_str() {
        "readme.md" | "readme.txt" | "readme" => "📖",
        "license" | "license.txt" | "license.md" => "📜",
        "changelog.md" | "changelog.txt" | "changelog" => "📋",
        "dockerfile" => "🐳",
        ".gitignore" => "🚫",
        "cargo.toml" | "cargo.lock" => "📦",
        "package.json" | "package-lock.json" => "📦",
        "makefile" => "🔨",
        _ => {
            if let Some(ext) = filename.split('.').last() {
                get_language_icon(ext)
            } else {
                "📄"
            }
        }
    }
}