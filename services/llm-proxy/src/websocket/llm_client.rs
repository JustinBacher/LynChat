use actix::Addr;
use log::{error, info};
use reqwest;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

use crate::error::{Error, empty_response_error};

use super::models::{
    CompletionRequest, StreamChunk, StreamEnd, StreamError, StreamMessage, StreamStart,
};
use super::session::ChatWebSocket;

impl ChatWebSocket {
    // Process a streaming request to llama.cpp
    pub async fn process_streaming_request(
        &self,
        prompt: &str,
        request_id: String,
        ctx_addr: Addr<ChatWebSocket>,
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let completion_url = format!("{}/completion", self.llamacpp_url);

        info!("Sending request to llama.cpp at {}", completion_url);
        info!("Prompt: {}", prompt);

        // Create the request payload with streaming enabled
        let payload = CompletionRequest {
            prompt: prompt.to_string(),
            n_predict: 512,
            temperature: 0.7,
            stream: true,
            stop: vec!["\n\nUSER:".to_string(), "\n\nASSISTANT:".to_string()],
            request_id: request_id.clone(),
        };

        // Send the request to llama.cpp
        info!("Sending request payload to llama.cpp: {:?}", payload);

        // Send a message to the client to indicate that streaming has started
        let start_message = StreamStart {
            msg_type: "stream_start".to_string(),
            model: "llama.cpp".to_string(),
            request_id: request_id.clone(),
        };

        self.send_message(&ctx_addr, &start_message, &request_id);

        // Use a longer timeout to allow for model loading and processing
        let response = client
            .post(&completion_url)
            .json(&payload)
            .timeout(Duration::from_secs(60)) // Increased timeout to 60 seconds
            .send()
            .await;

        match response {
            Ok(response) => self.handle_streaming_response(response, request_id, ctx_addr).await,
            Err(e) => self.handle_llm_error(e, completion_url, request_id, ctx_addr).await,
        }
    }

    // Handle streaming response from LLM
    async fn handle_streaming_response(
        &self,
        response: reqwest::Response,
        request_id: String,
        ctx_addr: Addr<ChatWebSocket>,
    ) -> Result<String, Error> {
        // Check if the response status is successful
        if !response.status().is_success() {
            let status = response.status();
            let error_msg = format!("LLM returned error status: {}", status);
            error!("{}", error_msg);

            let error = StreamError {
                msg_type: "stream_error".to_string(),
                error: error_msg.clone(),
                error_type: Some("http_error".to_string()),
                llamacpp_url: Some(format!("{}/completion", self.llamacpp_url)),
                request_id: request_id.clone(),
            };

            self.send_message(&ctx_addr, &error, &request_id);
            return Err(Error::LLMConnection(error_msg));
        }

        // Get the response as a stream of bytes
        let mut stream = response.bytes_stream();
        let mut accumulated_content = String::new();
        let mut buffer = String::new();
        let mut is_first_chunk = true;

        // Use futures_util to process the stream
        use futures_util::StreamExt;

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    // Convert chunk to string
                    match std::str::from_utf8(&chunk) {
                        Ok(chunk_str) => {
                            // Add to buffer and process complete lines
                            buffer.push_str(chunk_str);

                            // Process complete lines from the buffer
                            let lines: Vec<&str> = buffer.lines().collect();

                            // If the buffer doesn't end with a newline, the last line is incomplete
                            let has_incomplete_line = !buffer.ends_with('\n');

                            // Process all complete lines
                            let complete_lines_count = if has_incomplete_line && !lines.is_empty() {
                                lines.len() - 1
                            } else {
                                lines.len()
                            };

                            for i in 0..complete_lines_count {
                                let line = lines[i];
                                if line.starts_with("data: ") {
                                    let data = &line[6..]; // Skip "data: "

                                    // Check if this is the end of the stream
                                    if data == "[DONE]" {
                                        self.send_stream_end(&accumulated_content, &request_id, &ctx_addr);
                                        info!("Streaming completed for request ID: {}", request_id);
                                        return Ok(accumulated_content);
                                    }

                                    // Try to parse the data as JSON
                                    match serde_json::from_str::<Value>(data) {
                                        Ok(json_data) => {
                                            if let Some(content) = json_data.get("content").and_then(|c| c.as_str()) {
                                                // Add the content to the accumulated content
                                                accumulated_content.push_str(content);

                                                // Send the chunk to the client
                                                self.send_stream_chunk(content, is_first_chunk, &request_id, &ctx_addr);
                                                is_first_chunk = false;
                                            }
                                        },
                                        Err(e) => {
                                            // If we can't parse as JSON, log the error but continue processing
                                            let err = Error::JsonParsing(e);
                                            error!("Failed to parse chunk as JSON: {}", err);
                                            // We don't return the error here as we want to continue processing other chunks
                                        }
                                    }
                                }
                            }

                            // Keep any incomplete line in the buffer
                            if has_incomplete_line && !lines.is_empty() {
                                buffer = lines[lines.len() - 1].to_string();
                            } else {
                                buffer.clear();
                            }
                        },
                        Err(e) => {
                            // Log UTF-8 error but continue processing
                            error!("Failed to convert chunk to UTF-8: {}", e);
                        }
                    }
                },
                Err(e) => {
                    // Handle network error
                    let err = Error::Request(e);
                    error!("Error receiving stream chunk: {}", err);

                    let error = StreamError {
                        msg_type: "stream_error".to_string(),
                        error: format!("Error receiving stream chunk: {}", err),
                        error_type: Some("network_error".to_string()),
                        llamacpp_url: None,
                        request_id: request_id.clone(),
                    };

                    self.send_message(&ctx_addr, &error, &request_id);

                    // We don't return error here to allow sending the accumulated content
                    break;
                }
            }
        }

        // If we've accumulated any content, send a stream end message
        if !accumulated_content.is_empty() {
            self.send_stream_end(&accumulated_content, &request_id, &ctx_addr);
            info!("Streaming completed for request ID: {}", request_id);
            Ok(accumulated_content)
        } else {
            // If no content was accumulated, return an error
            let error_msg = "No content received from LLM".to_string();

            let error = StreamError {
                msg_type: "stream_error".to_string(),
                error: error_msg.clone(),
                error_type: Some("empty_response".to_string()),
                llamacpp_url: None,
                request_id: request_id.clone(),
            };

            self.send_message(&ctx_addr, &error, &request_id);
            Err(empty_response_error(error_msg))
        }
    }

    // Send stream end message
    fn send_stream_end(&self, content: &str, request_id: &str, ctx_addr: &Addr<ChatWebSocket>) {
        let end_message = StreamEnd {
            msg_type: "stream_end".to_string(),
            model: "llama.cpp".to_string(),
            content: content.to_string(),
            request_id: request_id.to_string(),
        };

        self.send_message(ctx_addr, &end_message, request_id);
    }

    // Send stream chunk message
    fn send_stream_chunk(
        &self,
        content: &str,
        is_first: bool,
        request_id: &str,
        ctx_addr: &Addr<ChatWebSocket>,
    ) {
        let chunk_message = StreamChunk {
            msg_type: "stream_chunk".to_string(),
            content: content.to_string(),
            is_first,
            request_id: request_id.to_string(),
        };

        self.send_message(ctx_addr, &chunk_message, request_id);
    }

    // Handle LLM request error
    async fn handle_llm_error(
        &self,
        e: reqwest::Error,
        completion_url: String,
        request_id: String,
        ctx_addr: Addr<ChatWebSocket>,
    ) -> Result<String, Error> {
        // Create a proper error using our error type
        let err = if e.is_timeout() {
            error!("Timeout error sending message to llama.cpp: {}", e);
            Error::Timeout(format!("Request to llama.cpp timed out: {}", e))
        } else if e.is_connect() {
            error!("Connection error to llama.cpp: {}", e);
            Error::LLMConnection(format!("Failed to connect to llama.cpp: {}", e))
        } else {
            error!("Error sending message to llama.cpp: {}", e);
            Error::Request(e)
        };

        // Provide more detailed error information for the client
        let error_msg = err.to_string();
        let error_type = match &err {
            Error::Timeout(_) => Some("timeout".to_string()),
            Error::LLMConnection(_) => Some("connection".to_string()),
            _ => Some("other".to_string()),
        };

        let error = StreamError {
            msg_type: "stream_error".to_string(),
            error: error_msg.clone(),
            error_type,
            llamacpp_url: if matches!(err, Error::LLMConnection(_)) {
                Some(completion_url)
            } else {
                None
            },
            request_id: request_id.clone(),
        };

        self.send_message(&ctx_addr, &error, &request_id);
        Err(err)
    }

    // Helper to send serialized messages
    pub fn send_message<T: Serialize>(
        &self,
        ctx_addr: &Addr<ChatWebSocket>,
        message: &T,
        request_id: &str,
    ) {
        let serialized = match serde_json::to_string(message) {
            Ok(json) => json,
            Err(e) => {
                // Create a proper error using our error type
                let err = Error::JsonParsing(e);
                error!("Failed to serialize message: {}", err);
                format!("{{\"error\": \"Internal error: {}\"}}", err)
            }
        };

        ctx_addr.do_send(StreamMessage {
            message: serialized,
            request_id: request_id.to_string(),
        });
    }
}
