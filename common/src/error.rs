use std::io::Error as IoError;

use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;
use url::ParseError as UrlParseError;

use crate::{config::ConfigError, llm::LLMError, memory::error::MemoryError, tools::ToolError};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("LLM error: {0}")]
    LLM(#[from] LLMError),

    #[error("Memory/Vector DB error")]
    Memory(#[from] MemoryError),

    #[error("Failed to parse tool call from LLM response: {0}")]
    ToolCallParseFailed(String),

    #[error("Requested tool not found: {0}")]
    ToolNotFound(String),

    #[error("Tool error: {0}")]
    Tool(#[from] ToolError),

    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Reqwest(#[from] ReqwestError),

    #[error("URL parse error: {0}")]
    Url(#[from] UrlParseError),

    #[error("An unexpected error occurred: {0}")]
    Other(String),

    #[error("Utility function error: {0}")]
    UtilsError(String),

    #[error(transparent)]
    ColorEyre(#[from] color_eyre::eyre::Error),
}
