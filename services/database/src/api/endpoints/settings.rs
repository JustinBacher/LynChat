// Settings endpoints module

use actix_web::{get, put, web, Responder, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::entities::Setting;

#[get("/settings/{user_id}")]
pub async fn get_settings(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let settings = Setting::find().filter(crate::entities::setting::Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match settings {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/settings")]
pub async fn update_setting(db: web::Data<DatabaseConnection>, item: web::Json<Setting>) -> impl Responder {
    let mut active: crate::entities::setting::ActiveModel = item.0.clone().into();
    let res = active.update(db.get_ref()).await;
    if let Ok(setting) = &res {
        // Optionally, call audit logging here if needed
    }
    match res {
        Ok(setting) => HttpResponse::Ok().json(setting),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}