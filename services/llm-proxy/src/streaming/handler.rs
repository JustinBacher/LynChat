use actix_web::web::Bytes;
use futures::{Stream, StreamExt};
use log::{debug, error, info};
use llama_cpp_2::{
    context::params::LlamaContextParams,
    llama_backend::LlamaBackend,
    llama_batch::LlamaBatch,
    model::{params::LlamaModelParams, AddBos, LlamaModel, Special},
    token::LlamaToken,
};
use std::{
    env,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use super::models::{StreamChunk, StreamChunkData, StreamEnd, StreamEndData, StreamStart, StreamStartData};
use crate::error::Error;

// Singleton for the LlamaBackend to ensure it's only initialized once
lazy_static::lazy_static! {
    static ref LLAMA_BACKEND: Mutex<Option<Arc<LlamaBackend>>> = Mutex::new(None);
}

// Get or initialize the LlamaBackend
fn get_llama_backend() -> Result<Arc<LlamaBackend>, Error> {
    let mut backend = LLAMA_BACKEND.lock().map_err(|e| Error::LLMConnection(format!("Failed to lock backend: {}", e)))?;

    if backend.is_none() {
        info!("Initializing llama.cpp backend");
        match LlamaBackend::init() {
            Ok(new_backend) => {
                *backend = Some(Arc::new(new_backend));
            }
            Err(e) => {
                error!("Failed to initialize llama.cpp backend: {}", e);
                return Err(Error::LLMConnection(format!("Failed to initialize llama.cpp backend: {}", e)));
            }
        }
    }

    Ok(backend.as_ref().unwrap().clone())
}

// Get the model path from environment variables
fn get_model_path() -> Result<PathBuf, Error> {
    let model_path = env::var("LYN_PROVIDER_CONFIGS_LLAMACPP_MODEL_PATH")
        .unwrap_or_else(|_| "./models/model.gguf".to_string());

    let path = Path::new(&model_path);
    if !path.exists() {
        return Err(Error::LLMConnection(format!("Model file not found at {}", model_path)));
    }

    Ok(path.to_path_buf())
}

// Load the model
fn load_model(backend: &Arc<LlamaBackend>) -> Result<LlamaModel, Error> {
    let model_path = get_model_path()?;
    info!("Loading model from {}", model_path.display());

    let model_params = LlamaModelParams::default();

    match LlamaModel::load_from_file(backend, &model_path, &model_params) {
        Ok(model) => {
            info!("Model loaded successfully");
            Ok(model)
        }
        Err(e) => {
            error!("Failed to load model: {}", e);
            Err(Error::LLMConnection(format!("Failed to load model: {}", e)))
        }
    }
}

// Create a context for the model
fn create_context(model: &LlamaModel, backend: &Arc<LlamaBackend>) -> Result<llama_cpp_2::context::LlamaContext, Error> {
    let ctx_params = LlamaContextParams::default()
        .with_n_ctx(std::num::NonZeroU32::new(2048))
        .with_n_batch(512);

    match model.new_context(backend, ctx_params) {
        Ok(ctx) => {
            info!("Context created successfully");
            Ok(ctx)
        }
        Err(e) => {
            error!("Failed to create context: {}", e);
            Err(Error::LLMConnection(format!("Failed to create context: {}", e)))
        }
    }
}

// Process a streaming request to the LLM
pub async fn process_streaming_request(
    prompt: &str,
) -> Result<impl Stream<Item = Result<Bytes, Error>>, Error> {
    info!("Processing streaming request with prompt: {}", prompt);

    // Get the backend
    let backend = get_llama_backend()?;

    // Load the model
    let model = load_model(&backend)?;

    // Create a context
    let mut ctx = create_context(&model, &backend)?;

    // Generate a request ID
    let request_id = Uuid::new_v4().to_string();

    // Tokenize the prompt
    let tokens = match model.str_to_token(prompt, AddBos::Always) {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("Failed to tokenize prompt: {}", e);
            return Err(Error::LLMParsing(format!("Failed to tokenize prompt: {}", e)));
        }
    };

    info!("Tokenized prompt into {} tokens", tokens.len());

    // Create a batch for the prompt
    let mut batch = match LlamaBatch::get_one(&tokens) {
        Ok(batch) => batch,
        Err(e) => {
            error!("Failed to create batch: {}", e);
            return Err(Error::LLMParsing(format!("Failed to create batch: {}", e)));
        }
    };

    // Decode the prompt
    if let Err(e) = ctx.decode(&mut batch) {
        error!("Failed to decode prompt: {}", e);
        return Err(Error::LLMParsing(format!("Failed to decode prompt: {}", e)));
    }

    // Create a stream start message
    let start_message = StreamStart {
        event: "start".to_string(),
        data: StreamStartData {
            type_: "stream_start".to_string(),
            model: "llama.cpp".to_string(),
            request_id: request_id.clone(),
        },
    };

    let start_json = match serde_json::to_string(&start_message) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize start message: {}", e);
            return Err(Error::JsonParsing(e));
        }
    };

    // Create a batch for generation
    let mut gen_batch = LlamaBatch::new(1, 1);

    // Get the last token from the prompt
    let last_token = tokens.last().cloned().unwrap_or_else(|| model.token_bos());

    // Add the last token to the generation batch
    if let Err(e) = gen_batch.add(last_token, 0, &[0], true) {
        error!("Failed to add token to batch: {}", e);
        return Err(Error::LLMParsing(format!("Failed to add token to batch: {}", e)));
    }

    // Create a stream for generation
    let mut accumulated_content = String::new();
    let mut is_first_chunk = true;
    let request_id_clone = request_id.clone();

    // Create a stream that will yield chunks of the generated text
    let stream = futures::stream::once(async move { Ok(Bytes::from(start_json)) })
        .chain(futures::stream::unfold(
            (ctx, gen_batch, model, accumulated_content, is_first_chunk, request_id_clone),
            move |(mut ctx, mut batch, model, mut accumulated_content, mut is_first_chunk, request_id)| async move {
                // Decode the batch
                if let Err(e) = ctx.decode(&mut batch) {
                    error!("Failed to decode batch: {}", e);
                    return Some((Err(Error::LLMParsing(format!("Failed to decode batch: {}", e))),
                                (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                }

                // Get the logits for the last token
                let token_data_array = ctx.token_data_array();

                // Sample the next token
                let token_id = match token_data_array.sample_token(0.7, 0.0, None, None, None) {
                    Ok(token) => token,
                    Err(e) => {
                        error!("Failed to sample token: {}", e);
                        return Some((Err(Error::LLMParsing(format!("Failed to sample token: {}", e))),
                                    (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                    }
                };

                // Convert the token to a string
                let token_str = match model.token_to_str(LlamaToken::new(token_id), Special::default()) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to convert token to string: {}", e);
                        return Some((Err(Error::LLMParsing(format!("Failed to convert token to string: {}", e))),
                                    (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                    }
                };

                debug!("Generated token: {}", token_str);

                // Add the token to the accumulated content
                accumulated_content.push_str(&token_str);

                // Create a chunk message
                let chunk_message = StreamChunk {
                    event: "chunk".to_string(),
                    data: StreamChunkData {
                        type_: "stream_chunk".to_string(),
                        content: token_str,
                        is_first: is_first_chunk,
                        request_id: request_id.clone(),
                    },
                };

                // Reset first chunk flag if needed
                if is_first_chunk {
                    is_first_chunk = false;
                }

                // Serialize to JSON
                let json_str = match serde_json::to_string(&chunk_message) {
                    Ok(json) => json,
                    Err(e) => {
                        error!("Failed to serialize chunk message: {}", e);
                        return Some((Err(Error::JsonParsing(e)),
                                    (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                    }
                };

                // Check if we should stop generation
                let should_stop = model.is_eog_token(LlamaToken::new(token_id)) ||
                                  token_id == model.token_eos().0 ||
                                  accumulated_content.len() > 4096;

                if should_stop {
                    // Create an end message
                    let end_message = StreamEnd {
                        event: "end".to_string(),
                        data: StreamEndData {
                            type_: "stream_end".to_string(),
                            content: accumulated_content.clone(),
                            request_id: request_id.clone(),
                        },
                    };

                    // Serialize to JSON
                    match serde_json::to_string(&end_message) {
                        Ok(json_str) => {
                            return Some((Ok(Bytes::from(json_str)),
                                        (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                        }
                        Err(e) => {
                            error!("Failed to serialize end message: {}", e);
                            return Some((Err(Error::JsonParsing(e)),
                                        (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                        }
                    }
                }

                // Clear the batch for the next token
                batch.clear();

                // Add the generated token to the batch
                if let Err(e) = batch.add(LlamaToken::new(token_id), 0, &[0], true) {
                    error!("Failed to add token to batch: {}", e);
                    return Some((Err(Error::LLMParsing(format!("Failed to add token to batch: {}", e))),
                                (ctx, batch, model, accumulated_content, is_first_chunk, request_id)));
                }

                Some((Ok(Bytes::from(json_str)), (ctx, batch, model, accumulated_content, is_first_chunk, request_id)))
            }
        ));

    Ok(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>)
}
