pub mod audit;
pub mod conversations;
pub mod settings;

use actix_web::{get, post, web, Responder, HttpResponse, HttpServer, App};

#[get("ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong.")
}

pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(get_settings)
            .service(update_setting)
            .service(get_conversations)
            .service(add_conversation)
            .service(get_audit_logs)
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}