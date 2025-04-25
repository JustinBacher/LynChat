use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, info};

// WebSocket session actor
pub struct ChatWebSocket;

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

// Handler for WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                debug!("Ping received");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                info!("Text message received: {}", text);
                // Echo the message back for now
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Binary message received: {} bytes", bin.len());
                // Echo the binary message back
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Connection closed");
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

// WebSocket route handler
pub async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // Extract chat ID from path if available
    let chat_id = req.match_info().get("id").unwrap_or("default");

    info!("WebSocket connection established for chat: {}", chat_id);

    // Log connection details for debugging
    if let Some(addr) = req.peer_addr() {
        debug!("Connection from IP: {}", addr);
    }

    // Start the WebSocket session
    ws::start(ChatWebSocket {}, &req, stream)
}
