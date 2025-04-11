//! Errors specific to memory operations (e.g., Qdrant interaction).

#[derive(thiserror::Error, Debug)]
pub enum MemoryError {
    #[error("Failed to connect to Qdrant: {0}")]
    Connection(String),

    #[error("Qdrant collection operation failed: {0}")]
    Collection(String),

    #[error(transparent)]
    Client(#[from] qdrant_client::QdrantError),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("LLM summarization failed: {0}")]
    Summarization(String), // Added for LYN-6

    #[error("Failed to process data from memory store: {0}")]
    DataProcessing(String),
}
