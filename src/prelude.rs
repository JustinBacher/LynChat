//! Crate prelude containing common types and traits.

// Generic Error type for the entire application
#[derive(thiserror::Error, Debug)]
pub enum Error {
    // TODO: Add specific error variants from modules here
    // Example: #[error("Configuration error: {0}")] Config(#[from] crate::config::Error),
    // Example: #[error("LLM error: {0}")] Llm(#[from] crate::llm::Error),

    // Generic I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // Placeholder for other top-level errors
    #[error("An unexpected error occurred: {0}")]
    Other(String),
}

// Generic Result type using the application's Error type
pub type Result<T> = std::result::Result<T, Error>;

// Commonly used standard library items (optional, but can be convenient)
pub use std::format as f;
pub use std::println as print; // Basic print for now

// Re-export tracing macros for convenience
pub use tracing::{debug, error, info, span, trace, warn, Level};
