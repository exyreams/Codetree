use super::{OutputGenerator, ProjectReport};

pub struct TextGenerator;

impl OutputGenerator for TextGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        
        // Header
        output.push_str("CODETREE PROJECT ANALYSIS\n");
        output.push_str("========================\n\n");
        output.push_str(&format!("Generated: {}\n\n", report.generated_at.format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Project info
        output.push_str(&report.project_info);
        output.push_str("\n");
        
        // File tree
        output.push_str("Project File Tree:\n");
        output.push_str("==================\n");
        output.push_str(&report.file_tree);
        output.push_str("\n");
        
        // Statistics
        output.push_str(&report.statistics.format_stats());
        output.push_str("\n");
        
        // File contents
        output.push_str("Project Files:\n");
        output.push_str("==============\n\n");
        
        for (i, file) in report.files.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, file.relative_path));
            
            if file.is_sensitive {
                output.push_str("   [SENSITIVE FILE - Content Protected]\n\n");
            } else if let Some(content) = &file.content {
                output.push_str("\n");
                output.push_str(content);
                output.push_str("\n");
            } else {
                output.push_str("   [Unable to read file content]\n");
            }
            output.push_str("\n");
        }
        
        Ok(output)
    }
    
    fn file_extension(&self) -> &'static str {
        "txt"
    }
}