//! Module for handling Lyn's memory, primarily interaction summaries.

pub mod error;
pub mod qdrant;
pub mod summarizer;

use serde_json::Value as JsonValue;

use crate::prelude::*;
pub use error::MemoryError;
pub use qdrant::{QdrantMemoryClient, VectorDbConfig};
pub use summarizer::summarize_interaction;

#[async_trait::async_trait]
pub trait MemoryClient: Send + Sync {
    /// Stores text (e.g., a summary) and optional metadata.
    /// The implementation is responsible for generating embeddings if needed.
    async fn store(&self, text: &str, metadata: Option<JsonValue>) -> Result<()>;

    /// Searches for relevant memories based on a query text.
    /// The implementation handles embedding the query and performing the search.
    /// Returns a list of (text, score) tuples.
    async fn search(&self, query: &str, limit: u64) -> Result<Vec<(String, f32)>>;

    /// Ensures the necessary storage (e.g., collection, table) exists.
    async fn ensure_collection(&self) -> Result<()>;
}
