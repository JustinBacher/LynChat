//! Error types for configuration handling.

use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Could not determine configuration directory")]
    DirectoryNotFound,

    #[error("Configuration file not found: {0}")]
    FileNotFound(String),

    #[error("Failed to read configuration file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse configuration: {0}")]
    ParseError(#[from] config::ConfigError),

    #[error("Configuration validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}
