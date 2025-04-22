pub mod endpoints;

use actix_web::{web, App, HttpServer};


use endpoints::{audit::get_audit_logs, conversations::{get_conversations, add_conversation}, health::health, settings::{get_settings, update_setting}, get_app};

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(get_app)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}