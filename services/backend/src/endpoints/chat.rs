use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
}

#[post("/api/chat")]
pub async fn chat_handler(msg: web::Json<ChatMessage>) -> impl Responder {
    HttpResponse::Ok().json(msg.0)
}
