use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing::info;

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");
    let db: sea_orm::DatabaseConnection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");
    tracing::info!("Connected to PostgreSQL database");

    api::main().await
}