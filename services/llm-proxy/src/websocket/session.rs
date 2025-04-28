use actix::{Actor, ActorContext, AsyncContext, Handler};
use actix_web_actors::ws;
use log::{debug, info, warn};
use std::env;
use std::time::{Duration, Instant};

use super::models::StreamMessage;

// WebSocket session actor
pub struct ChatWebSocket {
    pub llamacpp_url: String,
    // Track if we're currently processing a request
    pub processing: bool,
    // Track the current request ID
    pub current_request_id: Option<String>,
    // Track the last time we received a pong
    pub last_pong: Instant,
}

impl ChatWebSocket {
    pub fn new() -> Self {
        // Get the llama.cpp URL from environment variables or use default
        let host = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_URL")
            .unwrap_or_else(|_| "http://localhost".to_string());
        let port = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_PORT")
            .unwrap_or_else(|_| "8084".to_string());

        // Ensure the URL is properly formatted
        let host = if host.ends_with('/') {
            host[..host.len() - 1].to_string()
        } else {
            host
        };

        let llamacpp_url = format!("{}:{}", host, port);
        info!("Using llama.cpp URL: {}", llamacpp_url);

        Self {
            llamacpp_url,
            processing: false,
            current_request_id: None,
            last_pong: Instant::now(),
        }
    }

    // Heartbeat to keep the connection alive
    pub fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        // Create a heartbeat interval that sends pings and checks for pong responses
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            // Check if we've received a pong recently (within 15 seconds)
            if Instant::now().duration_since(act.last_pong) > Duration::from_secs(15) {
                // No pong received for too long, consider the connection dead
                warn!("No pong received for over 15 seconds, closing connection");
                ctx.stop();
                return;
            }

            // Send ping to keep the connection alive
            debug!("Sending heartbeat ping");
            ctx.ping(b"");

            // Check if we're still processing a request
            if act.processing {
                debug!("Still processing request ID: {:?}", act.current_request_id);
            }
        });
    }

    // Handle timeout for a request
    pub fn handle_timeout(&mut self, req_id: &str, ctx: &mut ws::WebsocketContext<Self>) {
        // Check if we're still processing the same request
        if let Some(current_req_id) = &self.current_request_id {
            if current_req_id == req_id {
                // Reset the processing flag
                self.processing = false;
                info!("Setting processing flag to false due to timeout (Request ID: {})", req_id);

                // Send error response to client
                let error = super::models::StreamError {
                    msg_type: "stream_error".to_string(),
                    error: format!("Request to llama.cpp timed out after 90 seconds"),
                    error_type: None,
                    llamacpp_url: None,
                    request_id: req_id.to_string(),
                };

                ctx.text(serde_json::to_string(&error).unwrap_or_else(|e| {
                    log::error!("Failed to serialize error response: {}", e);
                    format!("{{\"error\": \"Internal error: {}\"}}", e)
                }));
            }
        }
    }
}

// Implement handler for StreamMessage
impl Handler<StreamMessage> for ChatWebSocket {
    type Result = ();

    fn handle(&mut self, msg: StreamMessage, ctx: &mut ws::WebsocketContext<Self>) {
        // Check if this is an internal timeout message
        if msg.message.contains("internal_timeout") {
            // Parse the message to get the request ID
            if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&msg.message) {
                if let Some(req_id) = json_msg.get("request_id").and_then(|v| v.as_str()) {
                    self.handle_timeout(req_id, ctx);
                }
                return;
            }
        }

        // Send the message to the client
        ctx.text(msg.message);
    }
}

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // Set up heartbeat to keep connection alive
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket actor started");

        // Set up heartbeat to keep connection alive
        self.heartbeat(ctx);
    }

    // Handle actor stopping (connection closed)
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WebSocket actor stopped");

        // Clean up any resources or state
        self.processing = false;
        self.current_request_id = None;

        // Log if we were processing a request when the connection closed
        if let Some(req_id) = &self.current_request_id {
            warn!("Connection closed while processing request ID: {}", req_id);
        }
    }
}

// Make ChatWebSocket cloneable
impl Clone for ChatWebSocket {
    fn clone(&self) -> Self {
        Self {
            llamacpp_url: self.llamacpp_url.clone(),
            processing: self.processing,
            current_request_id: self.current_request_id.clone(),
            last_pong: self.last_pong,
        }
    }
}
