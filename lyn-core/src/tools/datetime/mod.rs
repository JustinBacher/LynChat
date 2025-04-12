use std::future::Future;

use crate::prelude::*;
use chrono::Local;
use ollama_rs::generation::tools::Tool as OllamaRsTool;
use schemars::JsonSchema;
use serde::Deserialize;

// Logic function
async fn get_current_datetime() -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("Executing date_time tool");
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    Ok(formatted_time)
}

// Parameters struct (empty for this tool)
#[derive(Deserialize, Debug, JsonSchema)]
pub struct DateTimeParams {}

/// Wrapper struct to implement the ollama_rs Tool trait for date/time.
#[derive(Debug, Clone, Default)]
pub struct DateTime;

impl OllamaRsTool for DateTime {
    // Define the associated parameter type
    type Params = DateTimeParams;

    fn name() -> &'static str {
        "date_time"
    }

    fn description() -> &'static str {
        "Provides the current local date and time. Takes no arguments."
    }

    fn call(
        &mut self,
        _params: Self::Params, // Parameters are ignored
    ) -> impl Future<Output = std::result::Result<String, Box<dyn std::error::Error + Send + Sync>>> {
        // Call the actual logic function
        get_current_datetime()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ollama_rs::generation::tools::Tool as _; // Import trait for methods

    #[async_std::test]
    async fn test_date_time_call() {
        let mut tool = DateTime;
        let params = DateTimeParams {}; // Empty params
        let result = tool.call(params).await;
        assert!(result.is_ok());

        let time_string = result.unwrap();
        assert!(!time_string.is_empty());
        assert!(time_string.contains('-'));
        assert!(time_string.contains(':'));
        assert!(time_string.contains(' '));

        let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
        assert!(re.is_match(&time_string));
    }

    #[test]
    fn test_date_time_static_info() {
        assert_eq!(DateTime::name(), "date_time");
        assert_eq!(DateTime::description(), "Provides the current local date and time. Takes no arguments.");
    }
}
