use crate::models::{GenerateChunk, ModelInfo};
use forge_core::{ForgeError, Result, TokenMeter};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
    client: Client,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn health_check(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);
        match self.client.get(&url).send().await {
            Ok(res) => res.status().is_success(),
            Err(_) => false,
        }
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ForgeError::UpstreamUnavailable(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(ForgeError::UpstreamUnavailable(format!(
                "Ollama returned HTTP {}",
                resp.status()
            )));
        }

        #[derive(serde::Deserialize)]
        struct TagsResponse {
            models: Option<Vec<RawModel>>,
        }

        #[derive(serde::Deserialize)]
        struct RawModel {
            name: String,
            modified_at: Option<String>,
            size: Option<u64>,
            digest: Option<String>,
            details: Option<RawDetails>,
        }

        #[derive(serde::Deserialize)]
        struct RawDetails {
            parameter_size: Option<String>,
            quantization_level: Option<String>,
        }

        let tags: TagsResponse = resp
            .json()
            .await
            .map_err(|e| ForgeError::UpstreamUnavailable(e.to_string()))?;
        let models = tags
            .models
            .unwrap_or_default()
            .into_iter()
            .map(|m| ModelInfo {
                name: m.name,
                modified_at: m.modified_at,
                size: m.size.unwrap_or(0),
                digest: m.digest.unwrap_or_default(),
                parameter_size: m.details.as_ref().and_then(|d| d.parameter_size.clone()),
                quantization_level: m.details.as_ref().and_then(|d| d.quantization_level.clone()),
            })
            .collect();

        Ok(models)
    }

    pub fn mock_stream_response(model: &str, prompt: &str) -> (Vec<GenerateChunk>, TokenMeter) {
        let words = [
            "Sovereign", "Conduit", "gateway", "active.", "Synthesizing", "response",
            "for:", prompt, "over", "zero-open-inbound-port", "tunnel.",
        ];

        let mut chunks = Vec::new();
        let mut meter = TokenMeter::new();
        meter.set_prompt_tokens(prompt.split_whitespace().count().max(1));

        for (i, word) in words.iter().enumerate() {
            let chunk_text = format!("{} ", word);
            meter.record_chunk(1, chunk_text.len());
            chunks.push(GenerateChunk {
                model: model.to_string(),
                response: chunk_text,
                done: false,
                total_duration: None,
                prompt_eval_count: Some(meter.snapshot().prompt_tokens),
                eval_count: Some(i + 1),
            });
        }

        chunks.push(GenerateChunk {
            model: model.to_string(),
            response: String::new(),
            done: true,
            total_duration: Some(150_000_000),
            prompt_eval_count: Some(meter.snapshot().prompt_tokens),
            eval_count: Some(words.len()),
        });

        (chunks, meter)
    }
}
