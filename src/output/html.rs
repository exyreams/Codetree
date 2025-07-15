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
        :root {
            --bg-primary: #0d1117;
            --bg-secondary: #161b22;
            --bg-tertiary: #21262d;
            --bg-hover: #30363d;
            --border-primary: #30363d;
            --border-secondary: #21262d;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --text-muted: #656d76;
            --accent-primary: #58a6ff;
            --accent-secondary: #1f6feb;
            --accent-gradient: linear-gradient(135deg, #58a6ff 0%, #1f6feb 100%);
            --success: #238636;
            --warning: #d29922;
            --danger: #da3633;
            --shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
            --shadow-hover: 0 12px 40px rgba(0, 0, 0, 0.6);
            --border-radius: 12px;
            --border-radius-sm: 8px;
            --font-mono: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', Consolas, monospace;
            --font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
        }
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: var(--font-sans);
            line-height: 1.6;
            color: var(--text-primary);
            background: var(--bg-primary);
            min-height: 100vh;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 24px;
        }
        
        .header {
            background: var(--accent-gradient);
            color: white;
            padding: 3rem 2rem;
            border-radius: var(--border-radius);
            margin-bottom: 2rem;
            text-align: center;
            box-shadow: var(--shadow);
            position: relative;
            overflow: hidden;
        }
        
        .header::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse"><path d="M 10 0 L 0 0 0 10" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="0.5"/></pattern></defs><rect width="100" height="100" fill="url(%23grid)"/></svg>');
            opacity: 0.3;
        }
        
        .header-content {
            position: relative;
            z-index: 1;
        }
        
        .header h1 {
            font-size: 3rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        }
        
        .header .subtitle {
            opacity: 0.9;
            font-size: 1.2rem;
            font-weight: 300;
        }
        
        .nav {
            background: var(--bg-secondary);
            padding: 1.5rem;
            border-radius: var(--border-radius);
            margin-bottom: 2rem;
            box-shadow: var(--shadow);
            border: 1px solid var(--border-primary);
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
            align-items: center;
        }
        
        .nav-btn {
            padding: 0.75rem 1.5rem;
            background: var(--bg-tertiary);
            color: var(--text-primary);
            border: 1px solid var(--border-primary);
            border-radius: var(--border-radius-sm);
            cursor: pointer;
            transition: all 0.3s ease;
            font-weight: 500;
            font-size: 0.95rem;
        }
        
        .nav-btn:hover {
            background: var(--bg-hover);
            border-color: var(--accent-primary);
            transform: translateY(-1px);
        }
        
        .nav-btn.active {
            background: var(--accent-gradient);
            border-color: var(--accent-primary);
            box-shadow: 0 4px 12px rgba(88, 166, 255, 0.3);
        }
        
        .search-container {
            flex: 1;
            min-width: 250px;
            position: relative;
        }
        
        .search-input {
            width: 100%;
            padding: 0.75rem 1rem 0.75rem 2.5rem;
            background: var(--bg-tertiary);
            border: 1px solid var(--border-primary);
            border-radius: var(--border-radius-sm);
            font-size: 1rem;
            color: var(--text-primary);
            transition: all 0.3s ease;
        }
        
        .search-input:focus {
            outline: none;
            border-color: var(--accent-primary);
            box-shadow: 0 0 0 3px rgba(88, 166, 255, 0.1);
        }
        
        .search-input::placeholder {
            color: var(--text-muted);
        }
        
        .search-icon {
            position: absolute;
            left: 0.75rem;
            top: 50%;
            transform: translateY(-50%);
            color: var(--text-muted);
            font-size: 1rem;
        }
        
        .section {
            background: var(--bg-secondary);
            margin-bottom: 2rem;
            border-radius: var(--border-radius);
            box-shadow: var(--shadow);
            border: 1px solid var(--border-primary);
            overflow: hidden;
            transition: all 0.3s ease;
        }
        
        .section:hover {
            box-shadow: var(--shadow-hover);
        }
        
        .section-header {
            background: var(--bg-tertiary);
            padding: 1.25rem 1.5rem;
            border-bottom: 1px solid var(--border-primary);
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            transition: all 0.3s ease;
        }
        
        .section-header:hover {
            background: var(--bg-hover);
        }
        
        .section-header h2 {
            color: var(--text-primary);
            font-size: 1.4rem;
            font-weight: 600;
        }
        
        .section-content {
            padding: 2rem;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }
        
        .stat-card {
            background: var(--bg-tertiary);
            border: 1px solid var(--border-primary);
            padding: 2rem;
            border-radius: var(--border-radius);
            text-align: center;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }
        
        .stat-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: var(--accent-gradient);
        }
        
        .stat-card:hover {
            transform: translateY(-4px);
            box-shadow: var(--shadow-hover);
            border-color: var(--accent-primary);
        }
        
        .stat-number {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
            color: var(--accent-primary);
            font-family: var(--font-mono);
        }
        
        .stat-label {
            color: var(--text-secondary);
            font-size: 0.95rem;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        
        .file-tree {
            background: var(--bg-primary);
            padding: 1.5rem;
            border-radius: var(--border-radius-sm);
            font-family: var(--font-mono);
            white-space: pre-wrap;
            overflow-x: auto;
            border: 1px solid var(--border-primary);
            color: var(--text-secondary);
            line-height: 1.5;
        }
        
        .file-list {
            display: grid;
            gap: 1rem;
        }
        
        .file-item {
            border: 1px solid var(--border-primary);
            border-radius: var(--border-radius);
            overflow: hidden;
            background: var(--bg-tertiary);
            transition: all 0.3s ease;
        }
        
        .file-item:hover {
            border-color: var(--accent-primary);
            box-shadow: 0 4px 12px rgba(88, 166, 255, 0.1);
        }
        
        .file-header {
            background: var(--bg-secondary);
            padding: 1.25rem;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            transition: all 0.3s ease;
        }
        
        .file-header:hover {
            background: var(--bg-hover);
        }
        
        .file-path {
            font-weight: 600;
            color: var(--text-primary);
            font-family: var(--font-mono);
            font-size: 0.95rem;
        }
        
        .file-meta {
            font-size: 0.85rem;
            color: var(--text-muted);
            font-family: var(--font-mono);
        }
        
        .file-content {
            display: none;
        }
        
        .file-content.active {
            display: block;
        }
        
        .code-block {
            background: var(--bg-primary);
            border: 1px solid var(--border-primary);
            border-radius: var(--border-radius-sm);
            overflow: hidden;
        }
        
        .code-header {
            background: var(--bg-tertiary);
            padding: 0.75rem 1rem;
            font-size: 0.85rem;
            color: var(--text-secondary);
            border-bottom: 1px solid var(--border-primary);
            font-family: var(--font-mono);
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        
        .code-content {
            padding: 1.5rem;
            font-family: var(--font-mono);
            white-space: pre-wrap;
            line-height: 1.5;
            color: var(--text-primary);
            overflow-x: auto;
            font-size: 0.9rem;
        }
        
        .sensitive-file {
            background: rgba(210, 153, 34, 0.1);
            color: var(--warning);
            padding: 2rem;
            text-align: center;
            font-style: italic;
            border: 1px solid rgba(210, 153, 34, 0.3);
            border-radius: var(--border-radius-sm);
        }
        
        .extension-list {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1rem;
        }
        
        .extension-item {
            background: var(--bg-tertiary);
            padding: 1.5rem;
            border-radius: var(--border-radius-sm);
            border: 1px solid var(--border-primary);
            border-left: 4px solid var(--accent-primary);
            transition: all 0.3s ease;
        }
        
        .extension-item:hover {
            transform: translateX(4px);
            border-left-color: var(--accent-secondary);
            box-shadow: 0 4px 12px rgba(88, 166, 255, 0.1);
        }
        
        .extension-name {
            font-family: var(--font-mono);
            font-weight: 700;
            color: var(--accent-primary);
            font-size: 1.1rem;
            margin-bottom: 0.5rem;
        }
        
        .extension-stats {
            color: var(--text-secondary);
            font-size: 0.9rem;
        }
        
        .hidden {
            display: none !important;
        }
        
        .collapse-icon {
            transition: transform 0.3s ease;
            color: var(--text-secondary);
            font-size: 1.2rem;
        }
        
        .collapsed .collapse-icon {
            transform: rotate(-90deg);
        }
        
        .progress-bar {
            width: 100%;
            height: 8px;
            background: var(--bg-tertiary);
            border-radius: 4px;
            overflow: hidden;
            margin: 0.5rem 0;
        }
        
        .progress-fill {
            height: 100%;
            background: var(--accent-gradient);
            transition: width 0.3s ease;
        }
        
        @media (max-width: 768px) {
            .container {
                padding: 16px;
            }
            
            .header {
                padding: 2rem 1rem;
            }
            
            .header h1 {
                font-size: 2.2rem;
            }
            
            .nav {
                flex-direction: column;
                gap: 0.75rem;
            }
            
            .nav-btn {
                width: 100%;
                text-align: center;
            }
            
            .stats-grid {
                grid-template-columns: 1fr;
            }
            
            .extension-list {
                grid-template-columns: 1fr;
            }
        }
        
        @media (prefers-reduced-motion: reduce) {
            * {
                animation-duration: 0.01ms !important;
                animation-iteration-count: 1 !important;
                transition-duration: 0.01ms !important;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="header-content">
                <h1>🌳 {{title}}</h1>
                <div class="subtitle">Generated on {{generated_at}}</div>
            </div>
        </div>
        
        <div class="nav">
            <button class="nav-btn active" onclick="showSection('overview')">📊 Overview</button>
            <button class="nav-btn" onclick="showSection('structure')">🗂️ Structure</button>
            <button class="nav-btn" onclick="showSection('files')">📄 Files</button>
            <div class="search-container">
                <span class="search-icon">🔍</span>
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
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: {{statistics.code_percentage}}%"></div>
                        </div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{{statistics.total_size}}</div>
                        <div class="stat-label">Total Size</div>
                    </div>
                </div>
                
                {{#if project_info}}
                <div style="background: var(--bg-tertiary); padding: 1.5rem; border-radius: var(--border-radius-sm); margin-bottom: 2rem; border: 1px solid var(--border-primary); white-space: pre-line; color: var(--text-secondary); line-height: 1.6;">{{project_info}}</div>
                {{/if}}
                
                {{#if statistics.files_by_extension}}
                <h3 style="margin-bottom: 1.5rem; color: var(--text-primary); font-size: 1.3rem; font-weight: 600;">📁 Files by Type</h3>
                <div class="extension-list">
                    {{#each statistics.files_by_extension}}
                    <div class="extension-item">
                        <div class="extension-name">.{{extension}}</div>
                        <div class="extension-stats">{{count}} files • {{lines}} lines</div>
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