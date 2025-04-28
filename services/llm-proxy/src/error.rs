use thiserror::Error;
use std::str::Utf8Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use actix_web::error::Error as ActixError;
use std::io::Error as IoError;

/// Main error type for the llm-proxy service
#[derive(Error, Debug)]
pub enum Error {
    /// Error when connecting to LLM service
    #[error("LLM connection error: {0}")]
    LLMConnection(String),

    /// Error when processing LLM response
    #[error("LLM response error: {0}")]
    LLMResponse(String),

    /// Error when parsing LLM response
    #[error("LLM parsing error: {0}")]
    LLMParsing(String),

    /// Error when processing WebSocket messages
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Error when parsing JSON
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] SerdeJsonError),

    /// Error when making HTTP requests
    #[error("HTTP request error: {0}")]
    Request(#[from] ReqwestError),

    /// Error when converting bytes to UTF-8
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] Utf8Error),

    /// Error from Actix framework
    #[error("Actix error: {0}")]
    Actix(#[from] ActixError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    /// Timeout errors
    #[error("Request timed out: {0}")]
    Timeout(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Empty response from LLM
    #[error("Empty response from LLM: {0}")]
    EmptyResponse(String),

    /// Catch-all for other errors
    #[error("Other error: {0}")]
    Other(String),
}

/// Shorthand for creating an LLM connection error
pub fn llm_connection_error<S: Into<String>>(msg: S) -> Error {
    Error::LLMConnection(msg.into())
}

/// Shorthand for creating an LLM response error
pub fn llm_response_error<S: Into<String>>(msg: S) -> Error {
    Error::LLMResponse(msg.into())
}

/// Shorthand for creating an LLM parsing error
pub fn llm_parsing_error<S: Into<String>>(msg: S) -> Error {
    Error::LLMParsing(msg.into())
}

/// Shorthand for creating a WebSocket error
pub fn websocket_error<S: Into<String>>(msg: S) -> Error {
    Error::WebSocket(msg.into())
}

/// Shorthand for creating a timeout error
pub fn timeout_error<S: Into<String>>(msg: S) -> Error {
    Error::Timeout(msg.into())
}

/// Shorthand for creating a configuration error
pub fn config_error<S: Into<String>>(msg: S) -> Error {
    Error::Configuration(msg.into())
}

/// Shorthand for creating a generic error
pub fn other_error<S: Into<String>>(msg: S) -> Error {
    Error::Other(msg.into())
}

/// Shorthand for creating an empty response error
pub fn empty_response_error<S: Into<String>>(msg: S) -> Error {
    Error::EmptyResponse(msg.into())
}
