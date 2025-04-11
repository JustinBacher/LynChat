mod error;

use crate::prelude::*;
use crate::tools::{Tool, ToolArgs, ToolError, ToolResult};
use async_trait::async_trait;
use serde_json::json; // Added for json! macro
pub use error::CalculatorError;

#[derive(Debug, Clone, Default)] // Added Debug
pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        CalculatorTool
    }
}

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> String {
        "calculator".to_string()
    }

    fn description(&self) -> String {
        "Performs mathematical calculations. Takes a single string argument containing the expression to evaluate (e.g., '2 + 2 * 3').".to_string()
    }

    /// Returns the JSON schema for the calculator tool's arguments.
    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate (e.g., '2 + 2 * 4 / ( 5 - 3 )')."
                }
            },
            "required": ["expression"]
        })
        // Note: The current execute implementation expects a raw string, not a JSON object.
        // This schema assumes the LLM will call with {"expression": "2+2"}.
        // We need to adjust the execute method to handle this JSON object format.
    }

    async fn execute(&self, args: ToolArgs) -> Result<ToolResult> {
        // Adjust to extract the expression from the JSON object based on the schema
        let expression = args
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                Error::Tool(ToolError::InvalidArguments(
                    "Expected a JSON object with an 'expression' field containing a string."
                        .to_string(),
                ))
            })?;

        // Evaluate the expression using meval
        let result = meval::eval_str(expression)
            .map(|result| result.to_string())
            .map_err(|e| Error::Tool(ToolError::Calculator(e.into())))?;
        Ok(ToolResult::from(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[async_std::test]
    async fn test_calculator_tool_simple_addition() {
        let tool = CalculatorTool;
        let args = json!("2 + 2");
        let result = tool.execute(args).await.unwrap();
        assert_eq!(result, "4");
    }

    #[async_std::test]
    async fn test_calculator_tool_complex_expression() {
        let tool = CalculatorTool;
        let args = json!("(3 * 4) / 2 + 5 - 1");
        let result = tool.execute(args).await.unwrap();
        assert_eq!(result, "10"); // (12 / 2) + 5 - 1 = 6 + 5 - 1 = 10
    }

    #[async_std::test]
    async fn test_calculator_tool_with_floats() {
        let tool = CalculatorTool;
        let args = json!("1.5 * 2.0");
        let result = tool.execute(args).await.unwrap();
        assert_eq!(result, "3");
    }

    #[async_std::test]
    async fn test_calculator_tool_invalid_expression() {
        let tool = CalculatorTool;
        let args = json!("2 +"); // Invalid expression
        let result = tool.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Tool(ToolError::InvalidArguments(_)) => {} // Expected error type
            e => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[async_std::test]
    async fn test_calculator_tool_invalid_argument_type() {
        let tool = CalculatorTool;
        let args = json!({"expression": "2 + 2"}); // Incorrect argument format
        let result = tool.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Tool(ToolError::InvalidArguments(_)) => {} // Expected error type
            e => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[async_std::test]
    async fn test_calculator_tool_division_by_zero() {
        let tool = CalculatorTool;
        let args = json!("1 / 0");
        let result = tool.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Tool(ToolError::ExecutionFailed(_)) => {} // Expected error type for runtime calc error
            e => panic!("Unexpected error type: {:?}", e),
        }
    }
}
