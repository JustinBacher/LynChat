use qdrant_client::{
    Payload,
    qdrant::{PointId, PointStruct, Vectors, point_id::PointIdOptions},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub collection_name: String,
    pub vector: Vec<f32>,
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpsertRequest {
    pub collection_name: String,
    pub points: Vec<ApiPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PointIdKind {
    Uuid(String),
    Integer(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiPoint {
    pub id: PointIdKind,
    pub vector: Vec<f32>,
    pub payload: Option<Payload>,
}

impl From<ApiPoint> for PointStruct {
    fn from(api_point: ApiPoint) -> Self {
        PointStruct {
            id: Some(match api_point.id {
                PointIdKind::Uuid(s) => PointId {
                    point_id_options: Some(PointIdOptions::Uuid(s)),
                },
                PointIdKind::Integer(i) => PointId {
                    point_id_options: Some(PointIdOptions::Num(i)),
                },
            }),
            vectors: Some(Vectors::from(api_point.vector)),
            payload: api_point.payload.unwrap_or_default().into(), // Use default empty payload if None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentPayload {
    pub doc_id: String,
    pub text: String,
}

// Conversion from ApiPoint to PointStruct is implemented in routes.rs or main.rs as needed.

