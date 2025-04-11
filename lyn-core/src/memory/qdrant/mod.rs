//! Handles interaction with the Qdrant vector database.

mod config;

use qdrant_client::{
    Payload,
    Qdrant, // Keep Qdrant client import
    qdrant::{
        CreateCollection,
        Distance,
        PointStruct,
        SearchPoints,
        UpsertPoints,
        VectorParams,
        VectorsConfig,
        WithPayloadSelector,
        WithVectorsSelector, // Added selectors
        vectors_config::Config as VectorConfig,
    },
};
use serde_json::Value as JsonValue; // Import alias for metadata
use uuid::Uuid;

use super::MemoryClient;
use crate::{memory::error::MemoryError, prelude::*};

pub use config::VectorDbConfig;

use std::sync::Arc; // Import Arc

/// A client for interacting with the Qdrant database for memory operations.
#[derive(Clone)]
pub struct QdrantMemoryClient {
    client: Arc<Qdrant>, // Wrap client in Arc
}

const COLLECTION_NAME: &str = "lyn";

impl QdrantMemoryClient {
    /// Creates a new Qdrant client.
    pub async fn new(config: &VectorDbConfig) -> Result<Self> {
        info!("Connecting to Qdrant at {}", config.url);

        // 1. Connect to Qdrant
        // Use try_from_url for potential async DNS resolution if needed, or stick to from_url
        let client = Qdrant::from_url(config.url.as_str())
            .build()
            .map_err(|e| Error::Memory(MemoryError::from(e)))?;

        // Collection check/creation is deferred to ensure_collection trait method
        info!("Successfully connected to Qdrant.");

        Ok(Self {
            client: Arc::new(client), // Store Arc<Qdrant>
        })
    }

    // --- Internal Helper Methods (Keep existing logic but make private/rename) ---

    /// Internal helper: Checks if the collection exists, creates it if not.
    async fn ensure_collection_internal(&self) -> Result<()> {
        let collection_exists = self
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .map_err(|e| Error::Memory(MemoryError::from(e)))?;

        if !collection_exists {
            info!("Collection '{}' not found. Creating...", COLLECTION_NAME);
            // TODO: Make vector parameters configurable or use a more informed default.
            // TODO: Get vector size from embedding model config later.
            let vector_size: u64 = 768; // Example size, adjust as needed
            self.client
                .create_collection(CreateCollection {
                    collection_name: COLLECTION_NAME.to_string(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(VectorConfig::Params(VectorParams {
                            size: vector_size,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        })),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e| Error::Memory(MemoryError::from(e)))?;
            info!("Collection '{}' created successfully.", COLLECTION_NAME);
        } else {
            trace!("Collection '{}' already exists.", COLLECTION_NAME);
        }
        Ok(())
    }

    /// Internal helper: Stores text, its vector, and metadata.
    async fn store_summary_internal(
        &self,
        summary_text: &str,
        vector: Vec<f32>,
        metadata: Option<Payload>, // Keep Option<Payload> for internal use
    ) -> Result<()> {
        let point_id = Uuid::new_v4().to_string();
        debug!("Generated Qdrant point ID: {}", point_id);

        let mut payload = metadata.unwrap_or_default(); // Start with provided metadata or empty
        payload.insert("text", summary_text.to_string()); // Ensure text is included

        let point = PointStruct::new(point_id.clone(), vector, payload);

        trace!("Storing summary in Qdrant collection '{}'", COLLECTION_NAME);
        let upsert_request = UpsertPoints {
            collection_name: COLLECTION_NAME.to_string(),
            points: vec![point],
            wait: Some(true), // Wait for operation to complete
            ..Default::default()
        };
        self.client
            .upsert_points(upsert_request)
            .await
            .map_err(|e| Error::Memory(MemoryError::from(e)))?;

        debug!("Successfully stored summary with ID: {}", point_id);
        Ok(())
    }

    /// Internal helper: Retrieves summaries based on a query vector.
    async fn retrieve_relevant_summaries_internal(
        &self,
        query_vector: Vec<f32>,
        limit: u64, // Use u64 directly
    ) -> Result<Vec<(String, f32)>> {
        trace!(
            "Retrieving top {} relevant summaries from collection '{}'",
            limit, COLLECTION_NAME
        );

        let search_request = SearchPoints {
            collection_name: COLLECTION_NAME.to_string(),
            vector: query_vector,
            limit,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(
                    qdrant_client::qdrant::with_payload_selector::SelectorOptions::Enable(true),
                ),
            }),
            with_vectors: Some(WithVectorsSelector {
                selector_options: Some(
                    qdrant_client::qdrant::with_vectors_selector::SelectorOptions::Enable(false),
                ),
            }),
            ..Default::default() // Use default for other fields
        };

        let search_result = self
            .client
            .search_points(search_request)
            .await
            .map_err(|e| Error::Memory(MemoryError::from(e)))?;

        debug!(
            "Qdrant search returned {} results.",
            search_result.result.len()
        );

        search_result
            .result
            .into_iter()
            .map(|scored_point| {
                let score = scored_point.score;
                let payload = scored_point.payload;
                let summary_text = payload
                    .get("text")
                    .and_then(|value| value.as_str())
                    .map(String::from)
                    .ok_or_else(|| {
                        error!(
                            "Qdrant point missing 'text' field in payload: {:?}",
                            payload
                        );
                        Error::Memory(MemoryError::DataProcessing(
                            "Qdrant point payload missing 'text' field".into(),
                        ))
                    })?;
                trace!(
                    "Retrieved summary (score: {:.4}): {}",
                    score,
                    summary_text.chars().take(50).collect::<String>()
                );
                Ok((summary_text, score))
            })
            .collect()
    }

    // --- Embedding Generation (Placeholder) ---
    // TODO: Implement actual embedding generation (e.g., using a separate service or model)
    // This function is removed for now to resolve compiler errors.
    // async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> { ... }

    // Helper to convert JsonValue metadata to Qdrant Payload
    fn json_to_payload(json_value: Option<JsonValue>) -> Option<Payload> {
        match json_value {
            Some(JsonValue::Object(map)) => {
                let mut payload = Payload::new();
                for (key, value) in map {
                    // Convert serde_json::Value to qdrant_client::qdrant::Value
                    // This might require more sophisticated conversion based on types
                    match value {
                        JsonValue::Null => { /* Skip null */ }
                        JsonValue::Bool(b) => payload.insert(key, b),
                        JsonValue::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                payload.insert(key, i);
                            } else if let Some(f) = n.as_f64() {
                                payload.insert(key, f);
                            }
                        }
                        JsonValue::String(s) => payload.insert(key, s),
                        JsonValue::Array(_) => {
                            warn!(
                                "Array metadata conversion not fully implemented for Qdrant Payload"
                            );
                        } // Handle arrays if needed
                        JsonValue::Object(_) => {
                            warn!(
                                "Nested object metadata conversion not fully implemented for Qdrant Payload"
                            );
                        } // Handle nested objects if needed
                    }
                }
                Some(payload)
            }
            _ => None, // Ignore non-object metadata or None
        }
    }
}

#[async_trait::async_trait]
impl MemoryClient for QdrantMemoryClient {
    /// Ensures the Qdrant collection exists.
    async fn ensure_collection(&self) -> Result<()> {
        self.ensure_collection_internal().await
    }

    /// Stores text and metadata in Qdrant. Embedding generation is currently omitted.
    async fn store(&self, text: &str, metadata: Option<JsonValue>) -> Result<()> {
        // TODO: 1. Generate embedding for the text here.
        let vector: Vec<f32> = vec![0.0; 768]; // Dummy vector
        warn!("Using dummy vector for store operation. Text: '{}'", text);

        // 2. Convert metadata if necessary (JsonValue -> Payload)
        let qdrant_payload = Self::json_to_payload(metadata);

        // 3. Call internal storage function
        self.store_summary_internal(text, vector, qdrant_payload)
            .await
    }

    /// Searches Qdrant based on query text. Embedding generation is currently omitted.
    async fn search(&self, query: &str, limit: u64) -> Result<Vec<(String, f32)>> {
        // TODO: 1. Generate embedding for the query here.
        let query_vector: Vec<f32> = vec![0.0; 768]; // Dummy vector
        warn!(
            "Using dummy vector for search operation. Query: '{}'",
            query
        );

        // 2. Call internal retrieval function
        self.retrieve_relevant_summaries_internal(query_vector, limit)
            .await
    }
}

// --- Unit Tests (Adapted for Trait Methods) ---
#[cfg(test)]
mod tests {
    use std::env;
    //
    use qdrant_client::qdrant::ScrollPoints;
    use serde_json::json;
    use url::Url;

    use super::*;

    // Helper to create a default config for tests
    fn test_config() -> Result<VectorDbConfig> {
        Ok(VectorDbConfig {
            url: Url::parse(
                &env::var("QDRANT_URL").unwrap_or("http://localhost:6334".to_string()),
            )?,
        })
    }

    // Helper function to clean up the test collection (operates on Arc<Qdrant>)
    async fn cleanup_test_collection(client: &Arc<Qdrant>, name: &str) {
        if client.collection_exists(name).await.unwrap_or(false) {
            client.delete_collection(name).await.unwrap();
            info!("Cleaned up test collection: {}", name);
        }
    }

    // Helper to create a client and ensure collection exists
    async fn setup_test_client() -> Result<QdrantMemoryClient> {
        let config = test_config()?;
        // Create client first to pass Arc<Qdrant> to cleanup
        let qdrant_client = Arc::new(
            Qdrant::from_url(config.url.as_str())
                .build()
                .expect("Failed to build Qdrant client for test setup"),
        );
        cleanup_test_collection(&qdrant_client, COLLECTION_NAME).await;

        // Now create the QdrantMemoryClient instance
        let memory_client = QdrantMemoryClient {
            client: qdrant_client, // Use the created Arc<Qdrant>
        };
        memory_client
            .ensure_collection()
            .await
            .expect("Ensure collection failed");
        Ok(memory_client)
    }

    #[async_std::test]
    #[ignore] // Ignore by default as it requires a running Qdrant instance
    async fn test_trait_ensure_collection() -> Result<()> {
        let config = test_config()?;
        // Create client first to pass Arc<Qdrant> to cleanup
        let qdrant_client_arc = Arc::new(
            Qdrant::from_url(config.url.as_str())
                .build()
                .map_err(MemoryError::from)?,
        );
        cleanup_test_collection(&qdrant_client_arc, COLLECTION_NAME).await;

        let client = QdrantMemoryClient {
            client: qdrant_client_arc.clone(),
        };

        // Call trait method
        client.ensure_collection().await?;
        // Use the Arc<Qdrant> directly for verification
        let exists = client
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .map_err(MemoryError::from)?;
        assert!(
            exists,
            "Collection should exist after ensure_collection call."
        );

        // Call again, should be idempotent
        client.ensure_collection().await?;
        let exists_again = client
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .map_err(MemoryError::from)?;
        assert!(
            exists_again,
            "Collection should still exist after second ensure_collection call."
        );

        cleanup_test_collection(&client.client, COLLECTION_NAME).await;
        Ok(())
    }

    #[async_std::test]
    #[ignore] // Ignore by default, requires running Qdrant
    async fn test_trait_store_and_verify() -> Result<()> {
        let client = setup_test_client().await?;
        let summary = "This is a test summary stored via trait.";
        let metadata = json!({ "timestamp": "2025-04-10T10:00:00Z", "type": "test", "value": 123 });

        // Call trait method
        let store_result = client.store(summary, Some(metadata)).await;
        assert!(
            store_result.is_ok(),
            "store failed: {:?}",
            store_result.err()
        );

        // Verification using scroll (as store doesn't return ID)
        let scroll_result = client
            .client
            .scroll(ScrollPoints {
                collection_name: COLLECTION_NAME.to_string(),
                limit: Some(1),
                with_payload: Some(true.into()),
                with_vectors: Some(false.into()), // Don't need vectors for verification
                ..Default::default()
            })
            .await
            .map_err(MemoryError::from)?;

        assert_eq!(
            scroll_result.result.len(),
            1,
            "Expected exactly one point after storing"
        );
        let retrieved_point = scroll_result.result.first().unwrap();
        let payload = &retrieved_point.payload; // Borrow payload

        assert!(payload.contains_key("text"));
        assert_eq!(payload["text"].as_str().unwrap(), summary);
        assert!(payload.contains_key("timestamp"));
        assert_eq!(
            payload["timestamp"].as_str().unwrap(),
            "2025-04-10T10:00:00Z"
        );
        assert!(payload.contains_key("type"));
        assert_eq!(payload["type"].as_str().unwrap(), "test");
        assert!(payload.contains_key("value"));
        assert_eq!(payload["value"].as_integer().unwrap(), 123); // Check integer

        cleanup_test_collection(&client.client, COLLECTION_NAME).await;
        Ok(())
    }

    #[async_std::test]
    #[ignore] // Ignore by default, requires running Qdrant
    async fn test_trait_search() -> Result<()> {
        let client = setup_test_client().await?;

        // Store some points using the trait method (relies on dummy embedding)
        client.store("Apple is a fruit", None).await?;
        client.store("Orange is also a fruit", None).await?;
        client.store("Car is a vehicle", None).await?;

        // Wait briefly for indexing
        async_std::task::sleep(std::time::Duration::from_millis(200)).await;

        let query = "Which things are fruits?";
        let limit = 2;

        // Call trait method
        let search_results = client.search(query, limit).await?;

        assert_eq!(
            search_results.len(),
            limit as usize,
            "Expected {} search results",
            limit
        );

        // Check contents (dummy embedding won't give meaningful order/results)
        info!("Search results (dummy embedding): {:?}", search_results);
        // Basic check that we got strings back
        assert!(!search_results[0].0.is_empty());
        assert!(!search_results[1].0.is_empty());

        cleanup_test_collection(&client.client, COLLECTION_NAME).await;
        Ok(())
    }
}
