//! Module for handling Lyn's memory, primarily interaction summaries.

pub mod error;

mod embedding;
mod summarizer;

pub use error::MemoryError;
pub use summarizer::summarize_interaction;
