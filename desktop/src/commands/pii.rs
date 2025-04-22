use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PIIDetection {
    #[serde(rename = "type")]
    detection_type: String,
    value: String,
    start_index: usize,
    end_index: usize,
    confidence: f32,
}

#[tauri::command]
pub async fn scan_for_pii(text: impl AsRef<str>) -> Result<Vec<PIIDetection>, String> {
    let mut detections = Vec::new();

    // Email pattern
    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    // Phone number pattern
    let _phone_regex = Regex::new(r"\b(\+\d{1,3}[ -]?)?\(?\d{3}\)?[ -]?\d{3}[ -]?\d{4}\b")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    // Credit card pattern
    let _cc_regex = Regex::new(r"\b(?:\d{4}[ -]?){3}\d{4}\b")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    // SSN pattern
    let _ssn_regex = Regex::new(r"\b\d{3}[ -]?\d{2}[ -]?\d{4}\b")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    // Check for emails
    for mat in email_regex.find_iter(text.as_ref()) {
        detections.push(PIIDetection {
            detection_type: "email".to_string(),
            value: text.as_ref()[mat.start()..mat.end()].to_string(),
            start_index: mat.start(),
            end_index: mat.end(),
            confidence: 0.95,
        });
    }

    // HACK: Don't use regex. This should be replaced with a more robust PII detection library.

    // Add phone numbers, credit cards, and SSNs
    // [Implementation for other PII types omitted for brevity]

    Ok(detections)
}

#[tauri::command]
pub async fn sanitize_text(
    text: impl AsRef<str>,
    mut detections: impl AsMut<[PIIDetection]>,
) -> Result<String, String> {
    if detections.as_mut().is_empty() {
        return Ok(text.as_ref().to_string());
    }

    // Sort detections by position in reverse order to avoid index shifting
    let mut sorted_detections = detections;
    sorted_detections
        .as_mut()
        .sort_by(|a, b| b.start_index.cmp(&a.start_index));

    let sanitized_text = text.as_ref();

    // Replace each detection with a placeholder
    for detection in sorted_detections.as_mut() {
        let replacement = match detection.detection_type.as_str() {
            "email" => "[EMAIL]",
            "phone_number" => "[PHONE]",
            "credit_card" => "[CREDIT_CARD]",
            "ssn" => "[SSN]",
            _ => "[REDACTED]",
        };

        sanitized_text
            .to_string()
            .replace_range(detection.start_index..detection.end_index, replacement);
    }

    Ok(sanitized_text.to_string())
}
