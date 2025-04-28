use actix::Message as ActixMessage;
use serde::{Serialize, Deserialize};

// Message type for streaming responses
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct StreamMessage {
    pub message: String,
    #[allow(dead_code)]
    pub request_id: String,
}

// Request message from client
#[derive(Deserialize, Debug)]
pub struct ClientRequest {
    pub message: Option<String>,
    pub prompt: Option<String>,
}

impl ClientRequest {
    // Get the message content, checking both fields
    pub fn get_prompt(&self) -> Option<String> {
        self.message.clone().or_else(|| self.prompt.clone())
    }
}

// LLM completion request
#[derive(Serialize, Debug)]
pub struct CompletionRequest {
    pub prompt: String,
    pub n_predict: i32,
    pub temperature: f32,
    pub stream: bool,
    pub stop: Vec<String>,
    pub request_id: String,
}

// Stream response types
#[derive(Serialize)]
pub struct StreamStart {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub model: String,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct StreamChunk {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub content: String,
    pub is_first: bool,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct StreamEnd {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub model: String,
    pub content: String,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct StreamError {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llamacpp_url: Option<String>,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct InternalTimeout {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct ProcessingStatus {
    pub status: String,
    pub message: String,
}
