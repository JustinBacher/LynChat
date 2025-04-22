mod search;
mod upsert;

use actix_web::{HttpResponse, Responder, get};

use common::prelude::*;

// Re-export endpoints
pub use search::search_embeddings;
pub use upsert::upsert_embeddings;

#[get("/health")]
pub(crate) async fn health_check() -> impl Responder {
    info!("Health check endpoint called");
    HttpResponse::Ok().body("Service is healthy")
}
