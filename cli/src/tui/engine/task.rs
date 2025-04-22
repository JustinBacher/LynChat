//! Engine task for processing prompts

use async_channel::{Receiver, Sender};
use async_std::task;
use common::core::Engine;
use common::prelude::*;
use futures_util::StreamExt;

use crate::tui::events::StreamEvent;

/// Spawns a task that processes prompts using the engine
pub fn spawn_engine_task(
    engine: Engine,
    prompt_rx: Receiver<String>,
    event_tx: Sender<StreamEvent>,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        info!("Engine task started.");
        // Clone the event sender for the task
        let event_tx_clone = event_tx.clone();
        while let Ok(prompt) = prompt_rx.recv().await {
            debug!("Engine task received prompt: '{}'", prompt);
            match engine.process_prompt_stream(prompt).await {
                Ok(mut stream) => {
                    while let Some(result) = stream.next().await {
                        let event = match result {
                            Ok(chunk) => StreamEvent::Chunk(chunk),
                            Err(e) => StreamEvent::Error(e.to_string()),
                        };
                        if event_tx_clone.send(event).await.is_err() {
                            warn!("Engine task failed to send stream event: TUI receiver dropped.");
                            // Break inner loop, outer loop will check recv again
                            break;
                        }
                    }
                    // Send End event after stream finishes (if receiver still exists)
                    if event_tx_clone.send(StreamEvent::End).await.is_err() {
                        warn!("Engine task failed to send End event: TUI receiver dropped.");
                    }
                }
                Err(e) => {
                    // Send Error event if stream creation failed
                    if event_tx_clone
                        .send(StreamEvent::Error(e.to_string()))
                        .await
                        .is_err()
                    {
                        warn!("Engine task failed to send Error event: TUI receiver dropped.");
                    }
                }
            }
        }
        info!("Engine task finished.");
    })
}
