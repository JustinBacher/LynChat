use actix_web::{web, Responder, HttpResponse, get};
use log::info;

// Import the websocket module
use crate::websocket;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    info!("Health check endpoint called");
    HttpResponse::Ok().body("LLM Proxy service is healthy")
}

// Configure all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
       .route("/ws/chat", web::get().to(websocket::chat_ws))
       .route("/ws/chat/{id}", web::get().to(websocket::chat_ws));
}
