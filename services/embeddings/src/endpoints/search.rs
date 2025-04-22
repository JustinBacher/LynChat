use actix_web::{HttpResponse, Responder, post, web};
use qdrant_client::{Qdrant, qdrant::SearchPoints};

use crate::models::SearchRequest;
use common::prelude::*;

#[post("/search")]
pub(crate) async fn search_embeddings(
    client: web::Data<Qdrant>,
    req: web::Json<SearchRequest>,
) -> impl Responder {
    info!(
        "Search endpoint called for collection: {}",
        req.collection_name
    );
    let search_request = SearchPoints {
        collection_name: req.collection_name.clone(),
        vector: req.vector.clone(),
        limit: req.limit,
        with_payload: Some(true.into()), // Include payload in results
        ..Default::default()
    };

    match client.search_points(search_request).await {
        Ok(response) => {
            info!("Search successful, found {} results", response.result.len());
            HttpResponse::Ok().json(
                response
                    .result
                    .into_iter()
                    .map(|point| point.payload)
                    .collect::<Vec<_>>(),
            )
        }
        Err(e) => {
            error!("Search failed: {}", e);
            HttpResponse::InternalServerError().body(format!("Search failed: {}", e))
        }
    }
}
