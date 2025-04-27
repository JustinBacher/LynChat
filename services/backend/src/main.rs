mod endpoints;
mod error;
mod prelude;

use endpoints::{chat, settings, ws};

use actix_files::Files;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Starting backend service...");

    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(chat::chat_handler)
            .service(settings::settings_handler)
            .service(ws::ws_chat_handler)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

