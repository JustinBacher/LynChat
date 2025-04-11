//! Crate prelude containing common types and traits.

// Generic Error type for the entire application
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError), // Corrected path

    #[error("LLM error: {0}")]
    LLM(#[from] crate::llm::LLMError),

    #[error("Memory/Vector DB error: {0}")]
    Memory(#[from] crate::memory::error::MemoryError),

    #[error("Failed to parse tool call from LLM response: {0}")]
    ToolCallParseFailed(String),

    #[error("Requested tool not found: {0}")]
    ToolNotFound(String),

    #[error("Tool error: {0}")] // Keep the original Tool error for specific tool impl errors
    Tool(#[from] crate::tools::ToolError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("An unexpected error occurred: {0}")]
    Other(String),

    #[error("Utility function error: {0}")]
    UtilsError(String),
}

// Generic Result type using the application's Error type
pub type Result<T> = std::result::Result<T, Error>;

// Geneneric Wrapper type
#[derive(Debug, Clone)]
pub struct W<T>(pub T);

// Commonly used standard library items (optional, but can be convenient)
pub use std::format as f;
pub use std::println as print; // Basic print for now

// Re-export tracing macros for convenience
#[allow(unused_imports)]
pub use tracing::{Level, debug, error, info, span, trace, warn};
