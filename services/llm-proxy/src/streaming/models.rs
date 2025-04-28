use serde::{Deserialize, Serialize};

// Request model for chat streaming
#[derive(Deserialize, Debug)]
pub struct ChatRequest {
    pub message: Option<String>,
    pub prompt: Option<String>,
    pub model: Option<String>,
    pub reasoning_enabled: Option<bool>,
}

impl ChatRequest {
    // Get the message content, checking both fields
    pub fn get_prompt(&self) -> Option<String> {
        self.message.clone().or_else(|| self.prompt.clone())
    }
}

// Response models for streaming
#[derive(Serialize, Debug)]
pub struct StreamStart {
    pub event: String,
    pub data: StreamStartData,
}

#[derive(Serialize, Debug)]
pub struct StreamStartData {
    pub type_: String,
    pub model: String,
    pub request_id: String,
}

#[derive(Serialize, Debug)]
pub struct StreamChunk {
    pub event: String,
    pub data: StreamChunkData,
}

#[derive(Serialize, Debug)]
pub struct StreamChunkData {
    pub type_: String,
    pub content: String,
    pub is_first: bool,
    pub request_id: String,
}

#[derive(Serialize, Debug)]
pub struct StreamEnd {
    pub event: String,
    pub data: StreamEndData,
}

#[derive(Serialize, Debug)]
pub struct StreamEndData {
    pub type_: String,
    pub content: String,
    pub request_id: String,
}

#[derive(Serialize, Debug)]
pub struct StreamError {
    pub event: String,
    pub data: StreamErrorData,
}

#[derive(Serialize, Debug)]
pub struct StreamErrorData {
    pub type_: String,
    pub error: String,
    pub request_id: String,
}
