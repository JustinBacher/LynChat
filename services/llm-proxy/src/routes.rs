use actix_web::{web, Responder, HttpResponse, get};
use log::info;

// Import the websocket and streaming modules
use crate::websocket;
use crate::streaming;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    info!("Health check endpoint called");
    HttpResponse::Ok().body("LLM Proxy service is healthy")
}

// Configure all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
       // WebSocket routes (legacy)
       .service(websocket::chat_ws_default)
       .service(websocket::chat_ws_with_id)
       // HTTP streaming routes (new)
       .service(streaming::stream_chat);
}
