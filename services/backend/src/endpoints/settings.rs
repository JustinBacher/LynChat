use actix_web::{web, Responder, HttpResponse, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub opt_in_storage: bool,
}

#[post("/api/settings")]
pub async fn settings_handler(settings: web::Json<Settings>) -> impl Responder {
    HttpResponse::Ok().json(settings.0)
}