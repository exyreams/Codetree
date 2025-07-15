use super::{OutputGenerator, ProjectReport};
use serde_json;

pub struct JsonGenerator;

impl OutputGenerator for JsonGenerator {
    fn generate(&self, report: &ProjectReport) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(report)?;
        Ok(json)
    }
    
    fn file_extension(&self) -> &'static str {
        "json"
    }
}