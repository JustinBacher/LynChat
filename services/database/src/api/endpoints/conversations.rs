// Conversations endpoints module

use actix_web::{get, post, web, Responder, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::entities::Conversation;

#[get("/conversations/{user_id}")]
pub async fn get_conversations(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let conversations = Conversation::find().filter(crate::entities::conversation::Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match conversations {
        Ok(convs) => HttpResponse::Ok().json(convs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/conversations")]
pub async fn add_conversation(db: web::Data<DatabaseConnection>, item: web::Json<Conversation>) -> impl Responder {
    let mut active: crate::entities::conversation::ActiveModel = item.0.clone().into();
    let res = active.insert(db.get_ref()).await;
    if let Ok(conv) = &res {
        // Optionally, call audit logging here if needed
    }
    match res {
        Ok(conv) => HttpResponse::Ok().json(conv),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}