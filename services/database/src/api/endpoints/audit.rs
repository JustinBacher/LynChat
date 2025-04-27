// Audit endpoints module

use actix_web::{get, web, Responder, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::audit_logs::{Entity as AuditLog, Column};

#[get("/audit/{user_id}")]
pub async fn get_audit_logs(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let logs = AuditLog::find().filter(Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match logs {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}