use std::env;
use sea_orm::Database;

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    // Connect to database
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");

    tracing::info!("Connected to PostgreSQL database");

    // Get port from environment variable or use default
    let port = env::var("DB_PORT").unwrap_or_else(|_| "8081".to_string());
    let port = port.parse::<u16>().expect("DB_PORT must be a valid port number");

    tracing::info!("Database service listening on port {}", port);

    // Start HTTP server using the run_server function from api module
    api::run_server(db, port).await
}