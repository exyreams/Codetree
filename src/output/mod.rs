pub mod formats;
pub mod html;
pub mod json;
pub mod markdown;

use crate::ProjectStats;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
    Html,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" | "txt" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "html" => Ok(OutputFormat::Html),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectReport {
    pub project_info: String,
    pub file_tree: String,
    pub statistics: ProjectStats,
    pub files: Vec<FileInfo>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub relative_path: String,
    pub content: Option<String>,
    pub is_sensitive: bool,
    pub size_bytes: u64,
    pub line_count: usize,
}

pub trait OutputGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>>;
    fn file_extension(&self) -> &'static str;
}