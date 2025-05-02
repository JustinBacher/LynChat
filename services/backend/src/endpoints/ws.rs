use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use tracing::info;

// This WebSocket handler is deprecated.
// The actual WebSocket implementation is in the llm-proxy service.
// This handler is kept for backward compatibility but will be removed in the future.
pub struct ChatWs;

impl Actor for ChatWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            // Log a warning that this WebSocket handler is deprecated
            info!("Received message on deprecated WebSocket handler: {}", text);

            // Send a response indicating that this endpoint is deprecated
            let response = format!("{{\"error\": \"This WebSocket endpoint is deprecated. Please use ws://localhost:8083/ws/chat instead.\", \"original_message\": {}}}", text);
            ctx.text(response);
        }
    }
}

#[get("/ws/chat")]
pub async fn ws_chat_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // Log a warning that this WebSocket handler is deprecated
    if let Some(addr) = req.peer_addr() {
        info!(
            "Connection to deprecated WebSocket endpoint from IP: {}",
            addr
        );
    }

    ws::start(ChatWs {}, &req, stream)
}
