use super::{OutputGenerator, ProjectReport};
use crate::ProjectStats;
use handlebars::Handlebars;
use serde_json::{json, Value};

pub struct HtmlGenerator;

impl OutputGenerator for HtmlGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();
        
        // Register the HTML template
        handlebars.register_template_string("report", HTML_TEMPLATE)?;
        
        // Prepare data for template
        let data = json!({
            "title": "Codetree Project Analysis",
            "generated_at": report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "project_info": report.project_info,
            "file_tree": report.file_tree,
            "statistics": {
                "total_files": report.statistics.total_files,
                "total_lines": report.statistics.total_lines,
                "code_lines": report.statistics.code_lines,
                "comment_lines": report.statistics.comment_lines,
                "blank_lines": report.statistics.blank_lines,
                "total_size": format_size(report.statistics.total_size_bytes),
                "code_percentage": if report.statistics.total_lines > 0 {
                    report.statistics.code_lines as f64 / report.statistics.total_lines as f64 * 100.0
                } else { 0.0 },
                "comment_percentage": if report.statistics.total_lines > 0 {
                    report.statistics.comment_lines as f64 / report.statistics.total_lines as f64 * 100.0
                } else { 0.0 },
                "blank_percentage": if report.statistics.total_lines > 0 {
                    report.statistics.blank_lines as f64 / report.statistics.total_lines as f64 * 100.0
                } else { 0.0 },
                "files_by_extension": prepare_extensions_data(&report.statistics)
            },
            "files": report.files.iter().map(|file| {
                json!({
                    "path": file.path,
                    "relative_path": file.relative_path,
                    "content": file.content,
                    "is_sensitive": file.is_sensitive,
                    "size_bytes": file.size_bytes,
                    "line_count": file.line_count,
                    "language": detect_language(&file.relative_path),
                    "size_formatted": format_size(file.size_bytes)
                })
            }).collect::<Vec<_>>()
        });
        
        let html = handlebars.render("report", &data)?;
        Ok(html)
    }
    
    fn file_extension(&self) -> &'static str {
        "html"
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

fn prepare_extensions_data(stats: &ProjectStats) -> Value {
    let mut extensions: Vec<_> = stats.files_by_extension.iter().collect();
    extensions.sort_by(|a, b| b.1.cmp(a.1));
    
    let extension_data: Vec<Value> = extensions.into_iter().map(|(ext, count)| {
        let lines = stats.lines_by_extension.get(ext).unwrap_or(&0);
        json!({
            "extension": ext,
            "count": count,
            "lines": lines
        })
    }).collect();
    
    Value::Array(extension_data)
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

const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f8f9fa;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 2rem;
            border-radius: 10px;
            margin-bottom: 2rem;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
        }
        
        .header .subtitle {
            opacity: 0.9;
            font-size: 1.1rem;
        }
        
        .nav {
            background: white;
            padding: 1rem;
            border-radius: 10px;
            margin-bottom: 2rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
        }
        
        .nav-btn {
            padding: 0.5rem 1rem;
            background: #667eea;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            transition: background 0.3s;
        }
        
        .nav-btn:hover {
            background: #5a6fd8;
        }
        
        .nav-btn.active {
            background: #764ba2;
        }
        
        .search-container {
            flex: 1;
            min-width: 200px;
        }
        
        .search-input {
            width: 100%;
            padding: 0.5rem;
            border: 2px solid #e9ecef;
            border-radius: 5px;
            font-size: 1rem;
        }
        
        .search-input:focus {
            outline: none;
            border-color: #667eea;
        }
        
        .section {
            background: white;
            margin-bottom: 2rem;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        .section-header {
            background: #f8f9fa;
            padding: 1rem 1.5rem;
            border-bottom: 1px solid #e9ecef;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        
        .section-header:hover {
            background: #e9ecef;
        }
        
        .section-header h2 {
            color: #495057;
            font-size: 1.3rem;
        }
        
        .section-content {
            padding: 1.5rem;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        
        .stat-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 1.5rem;
            border-radius: 8px;
            text-align: center;
        }
        
        .stat-number {
            font-size: 2rem;
            font-weight: bold;
            margin-bottom: 0.5rem;
        }
        
        .stat-label {
            opacity: 0.9;
            font-size: 0.9rem;
        }
        
        .file-tree {
            background: #f8f9fa;
            padding: 1rem;
            border-radius: 5px;
            font-family: 'Courier New', monospace;
            white-space: pre-wrap;
            overflow-x: auto;
            border: 1px solid #e9ecef;
        }
        
        .file-list {
            display: grid;
            gap: 1rem;
        }
        
        .file-item {
            border: 1px solid #e9ecef;
            border-radius: 8px;
            overflow: hidden;
        }
        
        .file-header {
            background: #f8f9fa;
            padding: 1rem;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        
        .file-header:hover {
            background: #e9ecef;
        }
        
        .file-path {
            font-weight: 500;
            color: #495057;
        }
        
        .file-meta {
            font-size: 0.9rem;
            color: #6c757d;
        }
        
        .file-content {
            display: none;
        }
        
        .file-content.active {
            display: block;
        }
        
        .code-block {
            background: #f8f9fa;
            border: 1px solid #e9ecef;
            border-radius: 5px;
            overflow-x: auto;
        }
        
        .code-header {
            background: #e9ecef;
            padding: 0.5rem 1rem;
            font-size: 0.9rem;
            color: #495057;
            border-bottom: 1px solid #dee2e6;
        }
        
        .code-content {
            padding: 1rem;
            font-family: 'Courier New', monospace;
            white-space: pre-wrap;
            line-height: 1.4;
        }
        
        .sensitive-file {
            background: #fff3cd;
            color: #856404;
            padding: 1rem;
            text-align: center;
            font-style: italic;
        }
        
        .extension-list {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 1rem;
        }
        
        .extension-item {
            background: #f8f9fa;
            padding: 1rem;
            border-radius: 5px;
            border-left: 4px solid #667eea;
        }
        
        .hidden {
            display: none !important;
        }
        
        .collapse-icon {
            transition: transform 0.3s;
        }
        
        .collapsed .collapse-icon {
            transform: rotate(-90deg);
        }
        
        @media (max-width: 768px) {
            .container {
                padding: 10px;
            }
            
            .header h1 {
                font-size: 2rem;
            }
            
            .nav {
                flex-direction: column;
            }
            
            .stats-grid {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🌳 {{title}}</h1>
            <div class="subtitle">Generated on {{generated_at}}</div>
        </div>
        
        <div class="nav">
            <button class="nav-btn active" onclick="showSection('overview')">📊 Overview</button>
            <button class="nav-btn" onclick="showSection('structure')">🗂️ Structure</button>
            <button class="nav-btn" onclick="showSection('files')">📄 Files</button>
            <div class="search-container">
                <input type="text" class="search-input" placeholder="Search files..." onkeyup="searchFiles(this.value)">
            </div>
        </div>
        
        <div id="overview-section" class="section">
            <div class="section-header" onclick="toggleSection('overview')">
                <h2>📊 Project Overview</h2>
                <span class="collapse-icon">▼</span>
            </div>
            <div class="section-content">
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-number">{{statistics.total_files}}</div>
                        <div class="stat-label">Total Files</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{{statistics.total_lines}}</div>
                        <div class="stat-label">Total Lines</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{{statistics.code_lines}}</div>
                        <div class="stat-label">Code Lines</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{{statistics.total_size}}</div>
                        <div class="stat-label">Total Size</div>
                    </div>
                </div>
                
                <div style="white-space: pre-line; margin-bottom: 2rem;">{{project_info}}</div>
                
                {{#if statistics.files_by_extension}}
                <h3 style="margin-bottom: 1rem;">📁 Files by Type</h3>
                <div class="extension-list">
                    {{#each statistics.files_by_extension}}
                    <div class="extension-item">
                        <strong>.{{extension}}</strong><br>
                        {{count}} files, {{lines}} lines
                    </div>
                    {{/each}}
                </div>
                {{/if}}
            </div>
        </div>
        
        <div id="structure-section" class="section">
            <div class="section-header" onclick="toggleSection('structure')">
                <h2>🗂️ Project Structure</h2>
                <span class="collapse-icon">▼</span>
            </div>
            <div class="section-content">
                <div class="file-tree">{{file_tree}}</div>
            </div>
        </div>
        
        <div id="files-section" class="section">
            <div class="section-header" onclick="toggleSection('files')">
                <h2>📄 File Contents</h2>
                <span class="collapse-icon">▼</span>
            </div>
            <div class="section-content">
                <div class="file-list">
                    {{#each files}}
                    <div class="file-item" data-file-path="{{relative_path}}">
                        <div class="file-header" onclick="toggleFile(this)">
                            <div class="file-path">{{relative_path}}</div>
                            <div class="file-meta">{{size_formatted}} • {{line_count}} lines</div>
                        </div>
                        <div class="file-content">
                            {{#if is_sensitive}}
                            <div class="sensitive-file">
                                🔒 SENSITIVE FILE - Content Protected
                            </div>
                            {{else}}
                            {{#if content}}
                            <div class="code-block">
                                <div class="code-header">{{language}}</div>
                                <div class="code-content">{{content}}</div>
                            </div>
                            {{else}}
                            <div class="sensitive-file">
                                ⚠️ Unable to read file content
                            </div>
                            {{/if}}
                            {{/if}}
                        </div>
                    </div>
                    {{/each}}
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function showSection(sectionName) {
            // Update nav buttons
            document.querySelectorAll('.nav-btn').forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
            
            // Show/hide sections
            document.querySelectorAll('.section').forEach(section => {
                if (section.id === sectionName + '-section') {
                    section.style.display = 'block';
                } else {
                    section.style.display = 'none';
                }
            });
        }
        
        function toggleSection(sectionName) {
            const section = document.getElementById(sectionName + '-section');
            const content = section.querySelector('.section-content');
            const header = section.querySelector('.section-header');
            
            if (content.style.display === 'none') {
                content.style.display = 'block';
                header.classList.remove('collapsed');
            } else {
                content.style.display = 'none';
                header.classList.add('collapsed');
            }
        }
        
        function toggleFile(header) {
            const content = header.nextElementSibling;
            content.classList.toggle('active');
        }
        
        function searchFiles(query) {
            const fileItems = document.querySelectorAll('.file-item');
            const lowerQuery = query.toLowerCase();
            
            fileItems.forEach(item => {
                const filePath = item.dataset.filePath.toLowerCase();
                if (filePath.includes(lowerQuery) || query === '') {
                    item.style.display = 'block';
                } else {
                    item.style.display = 'none';
                }
            });
        }
        
        // Initialize - show overview by default
        document.addEventListener('DOMContentLoaded', function() {
            showSection('overview');
        });
    </script>
</body>
</html>
"#;