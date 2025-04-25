use actix_web::{App, HttpServer};
use log::info;
use std::env;
use std::io::Result;

// Import modules
mod routes;
mod websocket;

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting llm-proxy service...");

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get port from environment variable or use default
    let port = env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");

    info!("LLM Proxy service listening on port {}", port);

    // Start HTTP server
    HttpServer::new(|| App::new().configure(routes::configure_routes))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
