use super::{OutputGenerator, ProjectReport};

pub struct MarkdownGenerator;

impl OutputGenerator for MarkdownGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        
        // Header
        output.push_str("# 🌳 Codetree Project Analysis\n\n");
        output.push_str(&format!("**Generated:** {}\n\n", report.generated_at.format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Project info
        output.push_str("## 📋 Project Information\n\n");
        for line in report.project_info.lines() {
            if !line.trim().is_empty() {
                output.push_str(&format!("{}\n", line));
            }
        }
        output.push_str("\n");
        
        // Statistics
        output.push_str("## 📊 Project Statistics\n\n");
        output.push_str(&format!("- **Total Files:** {}\n", report.statistics.total_files));
        output.push_str(&format!("- **Total Lines:** {}\n", report.statistics.total_lines));
        output.push_str(&format!("- **Code Lines:** {} ({:.1}%)\n", 
            report.statistics.code_lines,
            if report.statistics.total_lines > 0 {
                report.statistics.code_lines as f64 / report.statistics.total_lines as f64 * 100.0
            } else { 0.0 }
        ));
        output.push_str(&format!("- **Comment Lines:** {} ({:.1}%)\n", 
            report.statistics.comment_lines,
            if report.statistics.total_lines > 0 {
                report.statistics.comment_lines as f64 / report.statistics.total_lines as f64 * 100.0
            } else { 0.0 }
        ));
        output.push_str(&format!("- **Blank Lines:** {} ({:.1}%)\n", 
            report.statistics.blank_lines,
            if report.statistics.total_lines > 0 {
                report.statistics.blank_lines as f64 / report.statistics.total_lines as f64 * 100.0
            } else { 0.0 }
        ));
        
        let size_str = format_size(report.statistics.total_size_bytes);
        output.push_str(&format!("- **Total Size:** {}\n\n", size_str));
        
        // Files by extension
        if !report.statistics.files_by_extension.is_empty() {
            output.push_str("### 📁 Files by Type\n\n");
            let mut extensions: Vec<_> = report.statistics.files_by_extension.iter().collect();
            extensions.sort_by(|a, b| b.1.cmp(a.1));
            
            for (ext, count) in extensions {
                let lines = report.statistics.lines_by_extension.get(ext).unwrap_or(&0);
                output.push_str(&format!("- **.{}**: {} files, {} lines\n", ext, count, lines));
            }
            output.push_str("\n");
        }
        
        // File tree
        output.push_str("## 🗂️ Project Structure\n\n");
        output.push_str("```\n");
        output.push_str(&report.file_tree);
        output.push_str("```\n\n");
        
        // File contents
        output.push_str("## 📄 File Contents\n\n");
        
        for file in &report.files {
            output.push_str(&format!("### 📝 {}\n\n", file.relative_path));
            
            if file.is_sensitive {
                output.push_str("```\n[SENSITIVE FILE - Content Protected]\n```\n\n");
            } else if let Some(content) = &file.content {
                // Try to detect language for syntax highlighting
                let lang = detect_language(&file.relative_path);
                output.push_str(&format!("```{}\n", lang));
                output.push_str(content);
                if !content.ends_with('\n') {
                    output.push_str("\n");
                }
                output.push_str("```\n\n");
            } else {
                output.push_str("```\n[Unable to read file content]\n```\n\n");
            }
        }
        
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