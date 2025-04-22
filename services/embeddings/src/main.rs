mod endpoints;
mod models;
mod prelude;

use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use qdrant_client::Qdrant;
use tracing_subscriber::EnvFilter;

use common::prelude::*;
use endpoints::{search_embeddings, upsert_embeddings};

// Basic health check endpoint
#[get("/health")]
async fn health_check() -> impl Responder {
    info!("Health check endpoint called");
    HttpResponse::Ok().body("Service is healthy")
}

// Upsert endpoint

#[actix_web::main]
async fn main() -> IoResult<()> {
    // Load environment variables from .env file
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let qdrant_url = env::var("QDRANT_URL").expect("QDRANT_URL must be set");
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8082".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16");

    info!("Attempting to connect to Qdrant at: {}", qdrant_url);

    // Initialize Qdrant client
    // Note: Actual connection check happens on first request or specific check.
    // Consider adding a startup check if immediate validation is needed.
    let qdrant_client = match Qdrant::from_url(&qdrant_url).build() {
        Ok(client) => {
            info!(
                "Successfully initialized Qdrant client for URL: {}",
                qdrant_url
            );
            web::Data::new(client)
        }
        Err(e) => {
            error!("Failed to initialize Qdrant client: {}", e);
            // Depending on requirements, might panic or exit here
            // For now, we proceed but log the error. Operations requiring the client will fail.
            // Consider a more robust startup check or health endpoint validation.
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize Qdrant client: {}", e),
            ));
        }
    };

    info!(
        "Starting embeddings service at http://{}:{}/",
        server_host, server_port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(qdrant_client.clone()) // Share Qdrant client with handlers
            .service(health_check)
            .service(search_embeddings)
            .service(upsert_embeddings)
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
