// Settings endpoints module

use actix_web::{get, put, web, Responder, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait, ActiveModelTrait};
use crate::models::settings::{self, Entity as Setting, Column};
use serde_json::json;

#[get("/settings/{user_id}")]
pub async fn get_settings(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let settings = Setting::find().filter(Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match settings {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/settings")]
pub async fn update_setting(db: web::Data<DatabaseConnection>, item: web::Json<settings::Model>) -> impl Responder {
    // First check if the setting exists
    let setting = Setting::find_by_id(item.id).one(db.get_ref()).await;

    match setting {
        Ok(Some(_)) => {
            // Setting exists, update it
            let active = settings::ActiveModel {
                id: Set(item.id),
                user_id: Set(item.user_id),
                key: Set(item.key.clone()),
                value: Set(item.value.clone()),
                updated_at: Set(item.updated_at),
            };

            let res = active.update(db.get_ref()).await;
            match res {
                Ok(updated) => HttpResponse::Ok().json(updated),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        },
        Ok(None) => {
            // Setting doesn't exist, return not found
            HttpResponse::NotFound().json(json!({"error": "Setting not found"}))
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}