#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Failed to connect to LLM provider: {0}")]
    Connection(String),

    #[error("API request failed: {0}")]
    Api(String),

    #[error("Failed to parse LLM response: {0}")]
    Parsing(String),

    #[error("LLM provider configuration error: {0}")]
    Configuration(String),

    #[error("Missing LLM configuration for {0}")]
    Config(String),

    #[error("Error while embedding {0}. {1}")]
    Embedding(String, String)
}