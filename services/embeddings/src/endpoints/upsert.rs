use actix_web::{HttpResponse, Responder, post, web};
use qdrant_client::{
    Qdrant,
    qdrant::{
        CreateCollection, Distance, PointId, PointStruct, UpsertPoints, VectorParams, Vectors,
        point_id::PointIdOptions, vectors::VectorsOptions,
        vectors_config::Config as VectorsConfigParams,
    },
};

use crate::prelude::Result;
use common::prelude::*;

use crate::models::{ApiPoint, PointIdKind, UpsertRequest};

#[post("/upsert")]
pub(crate) async fn upsert_embeddings(
    client: web::Data<Qdrant>,
    request: web::Json<UpsertRequest>,
) -> impl Responder {
    if request.points.is_empty() {
        return HttpResponse::BadRequest().body("No points provided for upsert");
    }

    let points = match convert_points(&request.points) {
        Ok(points) => points,
        Err(e) => return e,
    };

    if let Err(e) = ensure_collection_exists(&client, &request.collection_name, &points).await {
        return e;
    };

    perform_upsert(&client, &request.collection_name, points).await
}

/// Convert API points to Qdrant point structures
fn convert_points(points: &[ApiPoint]) -> Result<Vec<PointStruct>> {
    points
        .iter()
        .map(|point| {
            Ok(PointStruct {
                id: Some(match &point.id {
                    PointIdKind::Uuid(s) => PointId {
                        point_id_options: Some(PointIdOptions::Uuid(s.clone())),
                    },
                    PointIdKind::Integer(i) => PointId {
                        point_id_options: Some(PointIdOptions::Num(*i)),
                    },
                }),
                vectors: Some(Vectors::from(point.vector.clone())),
                payload: point.payload.clone().map(Into::into).unwrap_or_default(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

/// Ensure collection exists or create it if it doesn't
async fn ensure_collection_exists(
    client: &Qdrant,
    collection_name: &str,
    points: &[PointStruct],
) -> Result<()> {
    match client.collection_info(collection_name).await {
        Ok(info) => {
            info!(
                "Collection '{}' exists. Status: {:?}",
                collection_name,
                info.result.unwrap_or_default()
            );
            Ok(())
        }
        Err(_) => {
            info!(
                "Collection '{}' not found, attempting to create.",
                collection_name
            );
            create_collection(client, collection_name, points).await
        }
    }
}

/// Create a new collection with appropriate vector configuration
async fn create_collection(
    client: &Qdrant,
    collection_name: &str,
    points: &[PointStruct],
) -> Result<()> {
    let vector_size = get_vector_size(points)?;

    client
        .create_collection(CreateCollection {
            collection_name: collection_name.to_string(),
            vectors_config: Some(qdrant_client::qdrant::VectorsConfig {
                config: Some(VectorsConfigParams::Params(VectorParams {
                    size: vector_size,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await
        .map_err(|e| {
            error!("Failed to create collection '{}': {}", collection_name, e);
            HttpResponse::InternalServerError().body(format!("Failed to create collection: {}", e))
        })?;

    info!("Successfully created collection '{}'", collection_name);
    Ok(())
}

fn get_vector_size(points: &[PointStruct]) -> Result<u64> {
    points
        .first()
        .and_then(|point| point.vectors.as_ref())
        .and_then(|vectors| match vectors.vectors_options.as_ref() {
            Some(VectorsOptions::Vector(v)) => Some(v.data.len() as u64),
            _ => None,
        })
        .ok_or_else(|| {
            HttpResponse::BadRequest()
                .body("Cannot determine vector size from first point's vector options")
        })
}

async fn perform_upsert(
    client: &Qdrant,
    collection_name: &str,
    points: Vec<PointStruct>,
) -> HttpResponse {
    client
        .upsert_points(UpsertPoints {
            collection_name: collection_name.to_string(),
            points,
            ..Default::default()
        })
        .await
        .map(|response| {
            info!(
                "Upsert successful: status {:?}, points affected: {}",
                response.result.as_ref().map(|r| r.status),
                response.result.as_ref().map_or(0, |r| r.status)
            );
            HttpResponse::Ok()
                .json(serde_json::json!({ "status": response.result.map(|r| r.status) }))
        })
        .unwrap_or_else(|e| {
            error!("Upsert failed: {}", e);
            HttpResponse::InternalServerError().body(format!("Upsert failed: {}", e))
        })
}
