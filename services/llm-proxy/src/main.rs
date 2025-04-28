use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use log::info;
use std::env;
use std::io::Result;

// Import modules
mod error;
mod prelude;
mod routes;
mod websocket;
mod streaming;

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting llm-proxy service...");

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get port from environment variable or use default
    let port = env::var("API_PORT").unwrap_or_else(|_| "8083".to_string());
    let port = port
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");

    // Always bind to 0.0.0.0 to ensure the service is accessible from outside the container
    info!("LLM Proxy service listening on 0.0.0.0:{}", port);

    // Start HTTP server
    let host = "0.0.0.0"; // Explicitly use 0.0.0.0 to bind to all interfaces
    info!("Binding to {}:{}", host, port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes::configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
