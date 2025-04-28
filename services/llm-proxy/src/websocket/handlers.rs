use actix::{ActorFutureExt, AsyncContext, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use bytes::Bytes;
use log::{debug, error, info, warn};
use serde_json;
use std::time::Duration;
use uuid::Uuid;

use super::models::{ClientRequest, ErrorResponse, InternalTimeout, ProcessingStatus, StreamMessage};
use super::session::ChatWebSocket;

// Handler for WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                info!("Ping received, responding with pong");
                // Update the last_pong timestamp when we receive a ping too
                // This helps keep the connection alive even if the client doesn't send pongs
                self.last_pong = std::time::Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                info!("Pong received from client");
                // Update the last_pong timestamp to keep the connection alive
                self.last_pong = std::time::Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                info!("Text message received: {}", text);
                // Update the last_pong timestamp when we receive any message
                self.last_pong = std::time::Instant::now();
                self.handle_text_message(text.to_string(), ctx);
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Binary message received: {} bytes", bin.len());
                // Update the last_pong timestamp when we receive any message
                self.last_pong = std::time::Instant::now();
                self.handle_binary_message(bin, ctx);
            }
            Ok(ws::Message::Close(reason)) => {
                // Update the last_pong timestamp when we receive any message
                self.last_pong = std::time::Instant::now();
                self.handle_close_request(reason, ctx);
            }
            _ => {
                debug!("Other message type received");
            }
        }
    }
}

impl ChatWebSocket {
    // Handle text message from client
    pub fn handle_text_message(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        // Set the processing flag to true
        self.processing = true;
        info!("Setting processing flag to true");

        // Parse the incoming message as JSON
        let client_request = match serde_json::from_str::<ClientRequest>(&text) {
            Ok(req) => req,
            Err(e) => {
                self.send_parse_error(e, ctx);
                return;
            }
        };

        // Extract the message content
        let prompt = match client_request.get_prompt() {
            Some(p) => p,
            None => {
                self.send_missing_field_error(ctx);
                return;
            }
        };

        self.process_client_request(prompt, ctx);
    }

    // Handle binary message from client
    pub fn handle_binary_message(&mut self, bin: Bytes, ctx: &mut ws::WebsocketContext<Self>) {
        // Set the processing flag to true
        self.processing = true;
        info!("Setting processing flag to true for binary message");

        // Try to parse the binary data as JSON
        let client_request = match serde_json::from_slice::<ClientRequest>(&bin) {
            Ok(req) => req,
            Err(e) => {
                self.send_binary_parse_error(e, ctx);
                return;
            }
        };

        // Extract the message content
        let prompt = match client_request.get_prompt() {
            Some(p) => p,
            None => {
                self.send_missing_field_error(ctx);
                return;
            }
        };

        self.process_client_request(prompt, ctx);
    }

    // Process a client request
    pub fn process_client_request(&mut self, prompt: String, ctx: &mut ws::WebsocketContext<Self>) {
        // Generate a request ID
        let request_id = Uuid::new_v4().to_string();
        self.current_request_id = Some(request_id.clone());

        // Set up timeout
        self.setup_request_timeout(request_id.clone(), ctx);

        // Clone necessary data for the async block
        let prompt_clone = prompt.clone();
        let request_id_clone = request_id.clone();
        let ctx_addr = ctx.address();
        let self_clone = self.clone();

        // Process the request in a separate async block
        actix::spawn(async move {
            // Process the streaming request
            match self_clone
                .process_streaming_request(&prompt_clone, request_id_clone.clone(), ctx_addr.clone())
                .await
            {
                Ok(response) => {
                    info!("Successfully processed request {}: {}", request_id_clone, response);
                },
                Err(e) => {
                    error!("Error processing request {}: {}", request_id_clone, e);
                }
            }
        });
    }

    // Set up a timeout for the request
    pub fn setup_request_timeout(&self, request_id: String, ctx: &mut ws::WebsocketContext<Self>) {
        let timeout_duration = Duration::from_secs(90); // 90 seconds timeout
        let timeout_ctx_addr = ctx.address();

        // Spawn a timeout future to reset the processing flag after a delay
        actix::spawn(async move {
            // Wait for the timeout duration
            actix::clock::sleep(timeout_duration).await;

            // Send a message to reset the processing flag
            let timeout_message = InternalTimeout {
                msg_type: "internal_timeout".to_string(),
                request_id: request_id.clone(),
            };

            let serialized = serde_json::to_string(&timeout_message).unwrap_or_else(|e| {
                format!(
                    "{{\"type\": \"internal_timeout\", \"request_id\": \"{}\", \"error\": \"{}\"}}",
                    request_id, e
                )
            });

            timeout_ctx_addr.do_send(StreamMessage {
                message: serialized,
                request_id,
            });
        });
    }

    // Handle close request
    pub fn handle_close_request(
        &mut self,
        reason: Option<ws::CloseReason>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        info!("Connection close request received");

        // Only close the connection if we're not processing a request
        if !self.processing {
            info!("Connection closed");
            ctx.close(reason);
            return;
        }

        info!("Received close request while processing a request");

        // Log the current request ID
        if let Some(req_id) = &self.current_request_id {
            info!("Currently processing request ID: {}", req_id);
        }

        // Force reset the processing flag after a longer delay
        self.setup_force_close(reason, ctx);

        // Send a ping to keep the connection alive temporarily
        ctx.ping(b"");

        // Send a message to the client to let them know we're still processing
        let status = ProcessingStatus {
            status: "processing".to_string(),
            message: "Still processing your request. Please wait...".to_string(),
        };

        ctx.text(
            serde_json::to_string(&status).unwrap_or_else(|_| {
                "{\"status\":\"processing\",\"message\":\"Still processing your request\"}".to_string()
            }),
        );
    }

    // Set up force close after delay
    pub fn setup_force_close(
        &mut self,
        reason: Option<ws::CloseReason>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        // Force reset the processing flag after a longer delay
        // This prevents hanging connections if a request is stuck
        // Increased from 30 to 60 seconds to be more lenient
        let force_close_fut = actix::clock::sleep(Duration::from_secs(60))
            .into_actor(self)
            .map(move |_, act, ctx| {
                if act.processing {
                    // Check if we have a current request ID
                    if let Some(req_id) = &act.current_request_id {
                        warn!("Request {} is still processing after 60 seconds, sending status update", req_id);

                        // Send a status update to the client instead of closing immediately
                        let status = ProcessingStatus {
                            status: "still_processing".to_string(),
                            message: "Your request is taking longer than expected. Still working on it...".to_string(),
                        };

                        ctx.text(
                            serde_json::to_string(&status).unwrap_or_else(|_| {
                                "{\"status\":\"still_processing\",\"message\":\"Your request is taking longer than expected\"}".to_string()
                            }),
                        );

                        // Set up another check after 30 more seconds
                        let final_close_fut = actix::clock::sleep(Duration::from_secs(30))
                            .into_actor(act)
                            .map(move |_, act, ctx| {
                                if act.processing {
                                    warn!("Forcing connection close after extended timeout (90 seconds total)");
                                    act.processing = false;
                                    ctx.close(reason);
                                } else {
                                    info!("Processing completed during extended wait time");
                                }
                            });

                        ctx.spawn(final_close_fut);
                    } else {
                        // No request ID but still processing - this is unusual
                        warn!("Processing flag is true but no request ID found, forcing connection close");
                        act.processing = false;
                        ctx.close(reason);
                    }
                } else {
                    info!("Processing completed, closing connection as requested");
                    ctx.close(reason);
                }
            });

        ctx.spawn(force_close_fut);
    }

    // Send parse error response
    pub fn send_parse_error(
        &mut self,
        e: serde_json::Error,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        error!("Failed to parse message as JSON: {}", e);

        // Send error response to client
        let error = ErrorResponse {
            error: format!("Failed to parse message: {}", e),
        };

        ctx.text(
            serde_json::to_string(&error).unwrap_or_else(|_| {
                format!("{{\"error\": \"Failed to parse message: {}\"}}", e)
            }),
        );

        // Set processing flag to false
        self.processing = false;
    }

    // Send binary parse error response
    pub fn send_binary_parse_error(
        &mut self,
        e: serde_json::Error,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        error!("Failed to parse binary message as JSON: {}", e);

        // Send error response to client
        let error = ErrorResponse {
            error: format!("Failed to parse binary message as JSON: {}", e),
        };

        ctx.text(
            serde_json::to_string(&error).unwrap_or_else(|_| {
                format!("{{\"error\": \"Failed to parse binary message as JSON: {}\"}}", e)
            }),
        );

        // Set processing flag to false
        self.processing = false;
    }

    // Send missing field error response
    pub fn send_missing_field_error(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let error_msg = "Message must contain either a 'message' or 'prompt' field with a string value";
        error!("{}", error_msg);

        // Send error response to client
        let error = ErrorResponse {
            error: error_msg.to_string(),
        };

        ctx.text(
            serde_json::to_string(&error)
                .unwrap_or_else(|_| format!("{{\"error\": \"{}\"}}", error_msg)),
        );

        // Set processing flag to false
        self.processing = false;
    }
}
