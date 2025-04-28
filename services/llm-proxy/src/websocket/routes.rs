use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, info};

use super::session::ChatWebSocket;

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
