use crate::models::WebhookRecord;
use forge_core::{ForgeError, Result};
use reqwest::Client;
use std::time::Duration;

pub async fn replay_webhook(record: &WebhookRecord, target_base_url: &str) -> Result<u16> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| ForgeError::Network(e.to_string()))?;

    let url = format!(
        "{}/{}",
        target_base_url.trim_end_matches('/'),
        record.path.trim_start_matches('/')
    );

    let mut req_builder = match record.method.to_uppercase().as_str() {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "PATCH" => client.patch(&url),
        "GET" => client.get(&url),
        _ => client.post(&url),
    };

    for (k, v) in &record.headers {
        if !k.eq_ignore_ascii_case("host") && !k.eq_ignore_ascii_case("content-length") {
            req_builder = req_builder.header(k, v);
        }
    }

    req_builder = req_builder.body(record.payload.clone());

    let resp = req_builder
        .send()
        .await
        .map_err(|e| ForgeError::Network(format!("Replay request failed: {}", e)))?;

    Ok(resp.status().as_u16())
}
