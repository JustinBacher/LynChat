use actix_web::web::Bytes;
use futures::Stream;
use log::{debug, error};
use serde_json::Value;
use std::env;
use std::pin::Pin;

use super::models::{
    StreamChunk, StreamChunkData, StreamEnd, StreamEndData, StreamError, StreamErrorData,
    StreamStart, StreamStartData,
};
use crate::error::Error;

/// Get the LLM URL from environment variables
pub fn get_llm_url() -> String {
    // Get the llama.cpp URL from environment variables or use default
    let host = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_URL")
        .unwrap_or_else(|_| "http://localhost".to_string());
    let port =
        env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_PORT").unwrap_or_else(|_| "8084".to_string());

    // Ensure the URL is properly formatted
    let host = if host.ends_with('/') {
        host[..host.len() - 1].to_string()
    } else {
        host
    };

    format!("{}:{}/completion", host, port)
}

/// Create an error response for streaming
pub fn create_error_response(error_msg: &str, request_id: &str) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>, Error> {
    // Create an error response
    let error_response = StreamError {
        event: "error".to_string(),
        data: StreamErrorData {
            type_: "stream_error".to_string(),
            error: error_msg.to_string(),
            request_id: request_id.to_string(),
        },
    };

    let error_json = serde_json::to_string(&error_response)
        .unwrap_or_else(|e| format!("{{\"error\": \"Serialization error: {}\"}}", e));

    // Return a stream with just the error message
    let bytes = Bytes::from(error_json);
    Ok(Box::pin(futures::stream::once(async move { Ok(bytes) }))
        as Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>)
}

/// Create a stream start message
pub fn create_start_message(request_id: &str) -> Result<String, Error> {
    let start_message = StreamStart {
        event: "start".to_string(),
        data: StreamStartData {
            type_: "stream_start".to_string(),
            model: "llama.cpp".to_string(),
            request_id: request_id.to_string(),
        },
    };

    serde_json::to_string(&start_message)
        .map_err(Error::JsonParsing)
}

/// Process a JSON value from the LLM stream
pub fn process_json_value(
    json_value: Value,
    chunk_str: &str,
    accumulated_content: &mut String,
    is_first_chunk: &mut bool,
    request_id: &str,
) -> Result<Bytes, Error> {
    // Check for content field
    if let Some(content) = json_value.get("content").and_then(|v| v.as_str()) {
        // Add to accumulated content
        accumulated_content.push_str(content);

        // Create a chunk message
        let chunk_message = StreamChunk {
            event: "chunk".to_string(),
            data: StreamChunkData {
                type_: "stream_chunk".to_string(),
                content: content.to_string(),
                is_first: *is_first_chunk,
                request_id: request_id.to_string(),
            },
        };

        // Reset first chunk flag if needed
        if *is_first_chunk {
            *is_first_chunk = false;
        }

        // Serialize to JSON
        match serde_json::to_string(&chunk_message) {
            Ok(json_str) => return Ok(Bytes::from(json_str)),
            Err(e) => {
                error!("Failed to serialize chunk message: {}", e);
                return Err(Error::JsonParsing(e));
            }
        }
    }

    // Check for stop field (end of stream)
    if json_value.get("stop").is_some() {
        let end_message = StreamEnd {
            event: "end".to_string(),
            data: StreamEndData {
                type_: "stream_end".to_string(),
                content: accumulated_content.clone(),
                request_id: request_id.to_string(),
            },
        };

        // Serialize to JSON
        match serde_json::to_string(&end_message) {
            Ok(json_str) => return Ok(Bytes::from(json_str)),
            Err(e) => {
                error!("Failed to serialize end message: {}", e);
                return Err(Error::JsonParsing(e));
            }
        }
    }

    // Unknown JSON format
    error!("Unknown JSON format: {}", chunk_str);
    Err(Error::LLMParsing(format!("Unknown JSON format: {}", chunk_str)))
}

/// Process a chunk from the LLM stream
pub fn process_chunk(
    chunk_result: Result<Bytes, reqwest::Error>,
    accumulated_content: &mut String,
    is_first_chunk: &mut bool,
    request_id: &str,
) -> Result<Bytes, Error> {
    // Handle request errors
    let chunk = match chunk_result {
        Ok(chunk) => chunk,
        Err(e) => {
            error!("Error getting chunk from stream: {}", e);
            return Err(Error::Request(e));
        }
    };

    // Convert bytes to UTF-8 string
    let chunk_str = match std::str::from_utf8(&chunk) {
        Ok(str) => str,
        Err(e) => {
            error!("Failed to parse chunk as UTF-8: {}", e);
            return Err(Error::Utf8(e));
        }
    };

    debug!("Received chunk: {}", chunk_str);

    // Parse string as JSON
    let json_value = match serde_json::from_str::<Value>(chunk_str) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to parse chunk as JSON: {}", e);
            return Err(Error::JsonParsing(e));
        }
    };

    // Process the JSON value
    process_json_value(json_value, chunk_str, accumulated_content, is_first_chunk, request_id)
}
