use actix::{Actor, AsyncContext, StreamHandler, ActorFutureExt, WrapFuture};
use actix_web::{web, Error, HttpRequest, HttpResponse, get};
use actix_web_actors::ws;
use log::{debug, info, error};
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use uuid::Uuid;

// WebSocket session actor
pub struct ChatWebSocket {
    llamacpp_url: String,
    // Track if we're currently processing a request
    processing: bool,
    // Track the current request ID
    current_request_id: Option<String>,
}

impl ChatWebSocket {
    pub fn new() -> Self {
        // Get the llama.cpp URL from environment variables or use default
        let host = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_URL")
            .unwrap_or_else(|_| "http://localhost".to_string());
        let port = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_PORT")
            .unwrap_or_else(|_| "8084".to_string());

        let llamacpp_url = format!("{}:{}", host, port);
        info!("Using llama.cpp URL: {}", llamacpp_url);

        Self {
            llamacpp_url,
            processing: false,
            current_request_id: None,
        }
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
}

impl ChatWebSocket {
    // Heartbeat to keep the connection alive
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            // Send ping to keep the connection alive
            debug!("Sending heartbeat ping");
            ctx.ping(b"");

            // Check if we're still processing a request
            if act.processing {
                debug!("Still processing request ID: {:?}", act.current_request_id);
            }
        });
    }
}

// Handler for WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                info!("Ping received, responding with pong");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                info!("Pong received from client");
            }
            Ok(ws::Message::Text(text)) => {
                info!("Text message received: {}", text);

                // Set the processing flag to true
                self.processing = true;
                info!("Setting processing flag to true");

                // IMPORTANT: This is the updated version that should NOT echo back messages with "Echo: " prefix
                // Clone necessary data for the async block
                let text_clone = text.clone();
                let llamacpp_url = self.llamacpp_url.clone();

                // Generate a request ID
                let request_id = Uuid::new_v4().to_string();
                self.current_request_id = Some(request_id.clone());

                // Create a separate function to handle the request
                let handle_request = async move {
                    info!("Processing message: {} (Request ID: {})", text_clone, request_id);

                    // Parse the incoming message as JSON
                    let parsed_message = match serde_json::from_str::<Value>(&text_clone) {
                        Ok(value) => value,
                        Err(e) => {
                            error!("Failed to parse message as JSON: {} (Request ID: {})", e, request_id);
                            return json!({
                                "error": format!("Failed to parse message: {}", e),
                                "request_id": request_id
                            }).to_string();
                        }
                    };

                    // Extract the message from the frontend format
                    let prompt = match parsed_message.get("message") {
                        Some(Value::String(p)) => p,
                        _ => {
                            // Try alternative field names
                            match parsed_message.get("prompt") {
                                Some(Value::String(p)) => p,
                                _ => {
                                    let error_msg = "Message must contain either a 'message' or 'prompt' field with a string value";
                                    error!("{}", error_msg);
                                    return json!({
                                        "error": error_msg
                                    }).to_string();
                                }
                            }
                        }
                    };

                    // Prepare the request to llama.cpp
                    let client = reqwest::Client::new();
                    let completion_url = format!("{}/completion", llamacpp_url);

                    info!("Sending request to llama.cpp at {}", completion_url);
                    info!("Prompt: {}", prompt);

                    // Create the request payload
                    let payload = json!({
                        "prompt": prompt,
                        "n_predict": 512,
                        "temperature": 0.7,
                        "stream": false,
                        "stop": ["\n\nUSER:", "\n\nASSISTANT:"], // Common stop sequences for chat models
                        "request_id": request_id
                    });

                    // Send the request to llama.cpp
                    match client
                        .post(&completion_url)
                        .json(&payload)
                        .timeout(Duration::from_secs(60))
                        .send()
                        .await {
                            Ok(response) => {
                                match response.json::<Value>().await {
                                    Ok(json_response) => {
                                        // Extract the content from the response
                                        match json_response.get("content") {
                                            Some(c) => {
                                                // Extract the content string
                                                let content_str = match c {
                                                    Value::String(s) => s.clone(),
                                                    _ => {
                                                        // If it's not a string, convert it to a string
                                                        let s = c.to_string();
                                                        // Remove quotes at the beginning and end if they exist
                                                        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                                                            s[1..s.len()-1].to_string()
                                                        } else {
                                                            s
                                                        }
                                                    }
                                                };

                                                info!("Received response from llama.cpp: {}", content_str);

                                                // Return the response in the expected format
                                                json!({
                                                    "content": content_str,
                                                    "model": "llama.cpp",
                                                    "stop_reason": json_response.get("stop_type").map_or("unknown", |v| v.as_str().unwrap_or("unknown"))
                                                }).to_string()
                                            },
                                            None => {
                                                error!("No content in llama.cpp response: {:?}", json_response);
                                                json!({
                                                    "error": "No content in llama.cpp response",
                                                    "raw_response": json_response
                                                }).to_string()
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("Failed to parse JSON response: {}", e);
                                        json!({
                                            "error": format!("Failed to parse JSON response: {}", e)
                                        }).to_string()
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Error sending message to llama.cpp: {}", e);
                                json!({
                                    "error": format!("Failed to communicate with llama.cpp: {}", e)
                                }).to_string()
                            }
                        }
                };

                // Use WrapFuture to convert the future to an actor future
                let fut = handle_request.into_actor(self);

                // Wait for the future to complete and send the response
                ctx.spawn(fut.map(|response, act, ctx| {
                    // Set processing flag to false
                    act.processing = false;
                    info!("Setting processing flag to false");

                    // Send the response
                    info!("Sending response back to client");
                    ctx.text(response);
                }));
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Binary message received: {} bytes", bin.len());

                // Set the processing flag to true
                self.processing = true;
                info!("Setting processing flag to true for binary message");

                // Generate a request ID for binary message
                let request_id = Uuid::new_v4().to_string();
                self.current_request_id = Some(request_id.clone());

                // Try to parse the binary data as JSON
                match serde_json::from_slice::<Value>(&bin) {
                    Ok(parsed_message) => {
                        // Handle the binary JSON message similar to text messages
                        let llamacpp_url = self.llamacpp_url.clone();

                        // Create a separate function to handle the request
                        let handle_request = async move {
                            // Extract the message from the parsed JSON
                            let prompt = match parsed_message.get("message").or_else(|| parsed_message.get("prompt")) {
                                Some(Value::String(p)) => p,
                                _ => {
                                    let error_msg = "Binary message must contain either a 'message' or 'prompt' field with a string value";
                                    error!("{}", error_msg);
                                    return json!({
                                        "error": error_msg
                                    }).to_string();
                                }
                            };

                            // Send to llama.cpp (reusing the same code pattern as for text messages)
                            let client = reqwest::Client::new();
                            let completion_url = format!("{}/completion", llamacpp_url);

                            info!("Sending binary request to llama.cpp at {}", completion_url);

                            // Create the request payload
                            let payload = json!({
                                "prompt": prompt,
                                "n_predict": 512,
                                "temperature": 0.7,
                                "stream": false,
                                "stop": ["\n\nUSER:", "\n\nASSISTANT:"],
                                "request_id": request_id
                            });

                            // Send the request to llama.cpp and process response
                            match client
                                .post(&completion_url)
                                .json(&payload)
                                .timeout(Duration::from_secs(60))
                                .send()
                                .await {
                                    Ok(response) => {
                                        match response.json::<Value>().await {
                                            Ok(json_response) => {
                                                // Extract the content from the response
                                                match json_response.get("content") {
                                                    Some(c) => {
                                                        // Extract the content string
                                                        let content_str = match c {
                                                            Value::String(s) => s.clone(),
                                                            _ => {
                                                                // If it's not a string, convert it to a string
                                                                let s = c.to_string();
                                                                // Remove quotes at the beginning and end if they exist
                                                                if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                                                                    s[1..s.len()-1].to_string()
                                                                } else {
                                                                    s
                                                                }
                                                            }
                                                        };

                                                        info!("Received binary response from llama.cpp: {}", content_str);

                                                        // Return the response in the expected format
                                                        json!({
                                                            "content": content_str,
                                                            "model": "llama.cpp",
                                                            "stop_reason": json_response.get("stop_type").map_or("unknown", |v| v.as_str().unwrap_or("unknown"))
                                                        }).to_string()
                                                    },
                                                    None => {
                                                        error!("No content in llama.cpp binary response: {:?}", json_response);
                                                        json!({
                                                            "error": "No content in llama.cpp response",
                                                            "raw_response": json_response
                                                        }).to_string()
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                error!("Failed to parse JSON response for binary request: {}", e);
                                                json!({
                                                    "error": format!("Failed to parse JSON response: {}", e)
                                                }).to_string()
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("Error sending binary message to llama.cpp: {}", e);
                                        json!({
                                            "error": format!("Failed to communicate with llama.cpp: {}", e)
                                        }).to_string()
                                    }
                                }
                        };

                        // Use WrapFuture to convert the future to an actor future
                        let fut = handle_request.into_actor(self);

                        // Wait for the future to complete and send the response
                        ctx.spawn(fut.map(|response, act, ctx| {
                            // Set processing flag to false
                            act.processing = false;
                            info!("Setting processing flag to false for binary message");

                            // Send the response
                            info!("Sending binary response back to client");
                            ctx.text(response);
                        }));
                    },
                    Err(e) => {
                        // If we can't parse as JSON, just echo the binary message back
                        error!("Failed to parse binary message as JSON: {}", e);

                        // Set processing flag to false
                        self.processing = false;
                        info!("Setting processing flag to false for unparseable binary message");

                        ctx.binary(bin);
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Connection close request received");

                // Only close the connection if we're not processing a request
                if !self.processing {
                    info!("Connection closed");
                    ctx.close(reason);
                } else {
                    info!("Ignoring close request while processing a request");
                    // Send a ping to keep the connection alive
                    ctx.ping(b"");
                }
            }
            _ => {
                debug!("Other message type received");
            },
        }
    }
}

// WebSocket route handler with macro for default chat
#[get("/ws/chat")]
pub async fn chat_ws_default(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("WebSocket connection established for default chat");

    // Log connection details for debugging
    if let Some(addr) = req.peer_addr() {
        debug!("Connection from IP: {}", addr);
    }

    // Start the WebSocket session
    ws::start(ChatWebSocket::new(), &req, stream)
}

// WebSocket route handler with macro for specific chat ID
#[get("/ws/chat/{id}")]
pub async fn chat_ws_with_id(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // Extract chat ID from path
    let chat_id = req.match_info().get("id").unwrap_or("default");

    info!("WebSocket connection established for chat: {}", chat_id);

    // Log connection details for debugging
    if let Some(addr) = req.peer_addr() {
        debug!("Connection from IP: {}", addr);
    }

    // Start the WebSocket session
    ws::start(ChatWebSocket::new(), &req, stream)
}
