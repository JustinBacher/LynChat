use futures_util::StreamExt;
use common::core::Engine;
use serde::Serialize;
use std::sync::Arc;
use tauri::{Emitter, Runtime, State, Window}; // Updated imports
use uuid::Uuid;

use super::pii::{sanitize_text, scan_for_pii};

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    id: String,
    content: String,
    complete: bool,
}

// Changed from #[tauri::command] to #[command]
#[tauri::command]
pub async fn send_message<R: Runtime>(
    engine: State<'_, Arc<Engine>>,
    window: Window<R>,
    message: String,
    auto_redact: bool,
) -> Result<String, String> {
    let message_id = Uuid::new_v4().to_string();
    let engine_clone = engine.inner().clone();

    let pii_detections = scan_for_pii(message.clone()).await?;
    if !pii_detections.is_empty() {
        // Updated event emission
        window
            .emit("chat:pii_detected", &pii_detections)
            .unwrap_or_default();

        // If auto-redact is enabled, sanitize the message
        if auto_redact {
            let sanitized = sanitize_text(message.clone(), pii_detections).await?;
            process_message(engine, window.clone(), sanitized, message_id.clone()).await;
        } else {
            return Ok(message_id);
        }
    } else {
        // No PII detected, process normally
        process_message(
            engine,
            window.clone(),
            message.clone(),
            message_id.clone(),
        )
        .await;
    }

    let message_id_clone = message_id.clone();
    // Updated spawn syntax
    let message_clone = message.clone();
    tauri::async_runtime::spawn(async move {
        match engine_clone.process_prompt_stream(message_clone).await {
            Ok(mut stream) => {
                let mut accumulated_content = String::new();

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(chunk) => {
                            accumulated_content.push_str(&chunk);

                            // Updated event emission
                            window
                                .emit(
                                    "chat:chunk",
                                    &ChatResponse {
                                        id: message_id_clone.clone(),
                                        content: accumulated_content.clone(),
                                        complete: false,
                                    },
                                )
                                .unwrap_or_default();
                        }
                        Err(e) => {
                            window
                                .emit("chat:error", format!("Error: {}", e))
                                .unwrap_or_default();
                            return;
                        }
                    }
                }

                // Updated event emission
                window
                    .emit(
                        "chat:complete",
                        &ChatResponse {
                            id: message_id_clone,
                            content: accumulated_content,
                            complete: true,
                        },
                    )
                    .unwrap_or_default();
            }
            Err(e) => {
                window
                    .emit("chat:error", format!("Failed to start stream: {}", e))
                    .unwrap_or_default();
            }
        }
    });

    Ok(message_id)
}

#[tauri::command]
pub async fn confirm_send_with_pii<R: Runtime>(
    engine: State<'_, Arc<Engine>>,
    window: Window<R>,
    message: String,
    message_id: String,
) -> Result<(), String> {
    process_message(engine, window, message, message_id).await;
    Ok(())
}

#[tauri::command]
pub async fn confirm_send_redacted<R: Runtime>(
    engine: State<'_, Arc<Engine>>,
    window: Window<R>,
    message: String,
    message_id: String,
) -> Result<(), String> {
    let pii_detections = scan_for_pii(message.clone()).await?;
    let sanitized = sanitize_text(message, pii_detections).await?;

    process_message(engine, window, sanitized, message_id).await;
    Ok(())
}

#[tauri::command]
pub async fn send_prompt(
    engine: State<'_, Arc<Engine>>,
    prompt: String,
) -> Result<ChatResponse, String> {
    match engine.process_prompt(&prompt).await {
        Ok(response) => Ok(ChatResponse {
            id: Uuid::new_v4().to_string(),
            content: response,
            complete: true,
        }),
        Err(e) => Err(format!("Failed to process prompt: {}", e)),
    }
}

async fn process_message<R: Runtime>(
    engine: State<'_, Arc<Engine>>,
    window: Window<R>,
    message: String,
    message_id: String,
) {
    // Updated event emission
    window
        .emit(
            "chat:start",
            serde_json::json!({
                "id": message_id,
                "message": message,
            }),
        )
        .unwrap_or_default();

    match engine.process_prompt_stream(message).await {
        Ok(mut stream) => {
            let mut accumulated_content = String::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        accumulated_content.push_str(&chunk);

                        // Updated event emission
                        window
                            .emit(
                                "chat:chunk",
                                &ChatResponse {
                                    id: message_id.clone(),
                                    content: accumulated_content.clone(),
                                    complete: false,
                                },
                            )
                            .unwrap_or_default();
                    }
                    Err(e) => {
                        window
                            .emit("chat:error", format!("Error: {}", e))
                            .unwrap_or_default();
                        return;
                    }
                }
            }

            // Updated event emission
            window
                .emit(
                    "chat:complete",
                    &ChatResponse {
                        id: message_id,
                        content: accumulated_content,
                        complete: true,
                    },
                )
                .unwrap_or_default();
        }
        Err(e) => {
            window
                .emit("chat:error", format!("Failed to start stream: {}", e))
                .unwrap_or_default();
        }
    }
}
