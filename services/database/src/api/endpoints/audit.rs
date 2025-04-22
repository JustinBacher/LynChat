// Audit endpoints module

use actix_web::{get, web, Responder, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::entities::AuditLog;

#[get("/audit/{user_id}")]
pub async fn get_audit_logs(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let logs = AuditLog::find().filter(crate::entities::audit_log::Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match logs {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}