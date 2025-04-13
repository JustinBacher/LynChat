use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Failed to connect to vector database: {0}")]
    Connection(String),

    #[error("Vector database collection operation failed: {0}")]
    Collection(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("LLM summarization failed: {0}")]
    Summarization(String),

    #[error("Failed to process data from memory store: {0}")]
    DataProcessing(String),

    #[error("Error while embedding {0}: {1}")]
    Embedding(String, String),
}
