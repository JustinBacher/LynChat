use super::{calculator::CalculatorError, datetime::DateTimeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Failed to execute tool: {0}")]
    ExecutionFailed(String),

    #[error("Invalid arguments provided to tool: {0}")]
    InvalidArguments(String),

    #[error("Tool '{0}' not found")]
    NotFound(String),

    #[error("Tool registration failed: {0}")]
    RegistrationFailed(String),

    #[error(transparent)]
    Calculator(#[from] CalculatorError),

    #[error(transparent)]
    DateTime(#[from] DateTimeError),
}
