// Conversations endpoints module

use actix_web::{get, post, web, Responder, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use crate::models::conversations::{self, Entity as Conversation, Column};
use serde_json::json;

#[get("/conversations/{user_id}")]
pub async fn get_conversations(db: web::Data<DatabaseConnection>, user_id: web::Path<i32>) -> impl Responder {
    let conversations = Conversation::find().filter(Column::UserId.eq(*user_id)).all(db.get_ref()).await;
    match conversations {
        Ok(convs) => HttpResponse::Ok().json(convs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/conversations")]
pub async fn add_conversation(db: web::Data<DatabaseConnection>, item: web::Json<conversations::Model>) -> impl Responder {
    let active = conversations::ActiveModel {
        user_id: Set(item.user_id),
        title: Set(item.title.clone()),
        messages: Set(item.messages.clone()),
        created_at: Set(item.created_at),
        updated_at: Set(item.updated_at),
        ..Default::default()
    };

    let res = Conversation::insert(active).exec(db.get_ref()).await;
    match res {
        Ok(insert_result) => {
            // Return the ID of the newly inserted record
            HttpResponse::Ok().json(json!({ "id": insert_result.last_insert_id }))
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}