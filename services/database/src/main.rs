use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing::info;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

mod api;
mod models;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Database service is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    // Connect to database
    let db: sea_orm::DatabaseConnection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");

    tracing::info!("Connected to PostgreSQL database");

    // Get port from environment variable or use default
    let port = env::var("DB_PORT").unwrap_or_else(|_| "8081".to_string());
    let port = port.parse::<u16>().expect("DB_PORT must be a valid port number");

    tracing::info!("Database service listening on port {}", port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(health_check)
            .service(api::endpoints::health)
            .service(api::endpoints::ping)
            .service(api::endpoints::audit::get_audit_logs)
            .service(api::endpoints::conversations::get_conversations)
            .service(api::endpoints::conversations::add_conversation)
            .service(api::endpoints::settings::get_settings)
            .service(api::endpoints::settings::update_setting)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}