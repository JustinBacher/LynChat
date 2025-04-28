use actix_web::{post, web, Error, HttpResponse, Responder};
use log::{error, info};
use uuid::Uuid;

use super::{handler::process_streaming_request, models::ChatRequest};

#[post("/stream/chat")]
pub async fn stream_chat(request: web::Json<ChatRequest>) -> Result<impl Responder, Error> {
    info!("Stream chat endpoint called");

    // Extract the message content
    let prompt = match request.get_prompt() {
        Some(p) => p,
        None => {
            error!("Message must contain either a 'message' or 'prompt' field");
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Message must contain either a 'message' or 'prompt' field with a string value"
            })));
        }
    };

    // Log model and reasoning settings if provided
    if let Some(model) = &request.model {
        info!("Model requested: {}", model);
    }

    if let Some(reasoning) = &request.reasoning_enabled {
        info!("Reasoning enabled: {}", reasoning);
    }

    // Generate a request ID
    let request_id = Uuid::new_v4().to_string();
    info!("Generated request ID: {}", request_id);

    // Process the streaming request
    match process_streaming_request(&prompt).await {
        Ok(stream) => {
            // Return a streaming response
            Ok(HttpResponse::Ok()
                .content_type("text/event-stream")
                .append_header(("Cache-Control", "no-cache"))
                .append_header(("Connection", "keep-alive"))
                .streaming(stream))
        }
        Err(e) => {
            error!("Error processing streaming request: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Error processing request: {}", e)
            })))
        }
    }
}
