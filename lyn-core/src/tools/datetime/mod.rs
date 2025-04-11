mod error;

use crate::{
    prelude::*,
    tools::{Tool, ToolArgs, ToolResult},
};
use async_trait::async_trait;
use chrono::Local;
use serde_json::json; // Added for json! macro

pub use error::DateTimeError;

/// A tool to provide the current date and time.
#[derive(Clone, Debug, Default)]
pub struct DateTimeTool;

impl DateTimeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for DateTimeTool {
    fn name(&self) -> String {
        String::from("date_time")
    }

    fn description(&self) -> String {
        String::from("Provides the current local date and time.")
    }

    /// Returns the JSON schema for the date_time tool's arguments (none needed).
    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {}, // No properties needed
            "required": []
        })
    }

    async fn execute(&self, _args: ToolArgs) -> Result<ToolResult> {
        // No argument parsing needed as per the schema.
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S %Z").to_string();
        Ok(formatted_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[async_std::test]
    async fn test_datetime_tool_execute() {
        let tool = DateTimeTool::new();
        let args = json!(null); // No arguments needed

        let result = tool.execute(args).await;
        assert!(result.is_ok());

        let time_string = result.unwrap();
        // Basic check: Ensure the string is not empty and contains typical date/time chars.
        assert!(!time_string.is_empty());
        assert!(time_string.contains('-')); // Date separator
        assert!(time_string.contains(':')); // Time separator
        assert!(time_string.contains(' ')); // Separator

        // We can't check the exact time, but we can check the format roughly.
        // Example: 2024-04-09 23:05:00 EST
        let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} [A-Z]+$").unwrap();
        // Note: The timezone abbreviation might vary (e.g., EST, EDT).
        // A more robust check might parse the date string back using chrono.
        // For now, this regex provides a basic format validation.
        assert!(re.is_match(&time_string));
        // Commenting out regex check as timezone abbreviations can be tricky/system-dependent.
        // The core functionality is getting *a* formatted time string.
    }

    #[test]
    fn test_datetime_tool_name() {
        let tool = DateTimeTool::new();
        assert_eq!(tool.name(), "date_time");
    }

    #[test]
    fn test_datetime_tool_description() {
        let tool = DateTimeTool::new();
        assert_eq!(
            tool.description(),
            "Provides the current local date and time."
        );
    }
}
