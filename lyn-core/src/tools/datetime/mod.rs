//! DateTime tool for retrieving current date and time

mod error;

use chrono::{Local, Utc};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::prelude::*;
pub(super) use error::DateTimeError;

/// DateTime tool for retrieving current date and time
#[derive(Debug, Clone)]
pub struct DateTime;

impl DateTime {
    pub async fn execute(&self, params: DateTimeParams) -> Result<String> {
        if params.utc {
            Ok(Utc::now()
                .format(params.format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S %Z"))
                .to_string())
        } else {
            Ok(Local::now()
                .format(params.format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S %Z"))
                .to_string())
        }
    }
}

/// Parameters for the DateTime tool
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DateTimeParams {
    /// Format to use for the date/time output (optional)
    #[serde(default)]
    pub format: Option<String>,

    /// Whether to use UTC instead of local time
    #[serde(default)]
    pub utc: bool,
}

impl Tool for DateTime {
    const NAME: &'static str = "datetime";

    type Error = DateTimeError;
    type Args = DateTimeParams;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Retrieves the current date and time. Can provide formatted output and support for UTC time."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "format": {
                        "type": "string",
                        "description": "Optional format string for the date/time output (e.g., %Y-%m-%d for YYYY-MM-DD)",
                    },
                    "utc": {
                        "type": "boolean",
                        "description": "Whether to use UTC instead of local time",
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> std::result::Result<Self::Output, Self::Error> {
        if args.utc {
            Ok(Utc::now()
                .format(args.format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S %Z"))
                .to_string())
        } else {
            Ok(Local::now()
                .format(args.format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S %Z"))
                .to_string())
        }
    }
}

impl ToolEmbedding for DateTime {
    type InitError = DateTimeError;
    type Context = ();
    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Calculator for mathematical expressions".to_string(),
            "Math calculations and arithmetic operations".to_string(),
            "Calculate numbers, evaluate expressions, perform math".to_string(),
            "Add, subtract, multiply, divide numbers".to_string(),
            "Trigonometric functions, logarithms, square roots".to_string(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(_state: Self::State, _context: Self::Context) -> StdResult<Self, Self::InitError> {
        Ok(DateTime)
    }
}
