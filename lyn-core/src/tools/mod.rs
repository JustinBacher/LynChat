//! Defines tools available to the Lyn AI assistant and related utilities.
//! Tools now conform to the `ollama_rs::generation::tools::Tool` trait.

// Module declarations for individual tools
pub mod calculator;
pub mod datetime;
// pub mod discovery; // Removed discovery module
pub mod error;

// Re-exports the concrete tool structs and the error type
pub use self::{
    calculator::Calculator, // Corrected export
    datetime::DateTime,     // Corrected export
    error::ToolError,
};

// Note: The internal `Tool` trait, `ToolRegistry`, and `OllamaToolAdapter`
// have been removed as we are now directly implementing the
// `ollama_rs::generation::tools::Tool` trait in each tool module.
