pub mod audit;
pub mod conversations;
pub mod settings;

use actix_web::{get, post, web, Responder, HttpResponse, HttpServer, App};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong.")
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Database service is healthy")
}

pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(ping)
            .service(settings::get_settings)
            .service(settings::update_setting)
            .service(conversations::get_conversations)
            .service(conversations::add_conversation)
            .service(audit::get_audit_logs)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}