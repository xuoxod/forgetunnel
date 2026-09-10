use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetrics {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub total_bytes: usize,
    pub ttft_ms: f64,
    pub total_duration_ms: f64,
    pub tokens_per_second: f64,
}

pub struct TokenMeter {
    start_time: Instant,
    first_token_time: Option<Instant>,
    prompt_tokens: usize,
    completion_tokens: usize,
    total_bytes: usize,
}

impl TokenMeter {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            first_token_time: None,
            prompt_tokens: 0,
            completion_tokens: 0,
            total_bytes: 0,
        }
    }

    pub fn set_prompt_tokens(&mut self, count: usize) {
        self.prompt_tokens = count;
    }

    pub fn record_chunk(&mut self, chunk_tokens: usize, bytes_len: usize) {
        if self.first_token_time.is_none() && chunk_tokens > 0 {
            self.first_token_time = Some(Instant::now());
        }
        self.completion_tokens += chunk_tokens;
        self.total_bytes += bytes_len;
    }

    pub fn record_first_token_now(&mut self) {
        if self.first_token_time.is_none() {
            self.first_token_time = Some(Instant::now());
        }
    }

    pub fn snapshot(&self) -> StreamMetrics {
        let total_duration = self.start_time.elapsed().as_secs_f64() * 1000.0;
        let ttft_ms = self
            .first_token_time
            .map(|t| t.duration_since(self.start_time).as_secs_f64() * 1000.0)
            .unwrap_or(total_duration);

        let gen_duration_sec = self
            .first_token_time
            .map(|t| t.elapsed().as_secs_f64())
            .unwrap_or(self.start_time.elapsed().as_secs_f64());

        let tokens_per_second = if self.completion_tokens > 0 {
            self.completion_tokens as f64 / gen_duration_sec.max(0.0001)
        } else {
            0.0
        };

        StreamMetrics {
            prompt_tokens: self.prompt_tokens,
            completion_tokens: self.completion_tokens,
            total_tokens: self.prompt_tokens + self.completion_tokens,
            total_bytes: self.total_bytes,
            ttft_ms,
            total_duration_ms: total_duration,
            tokens_per_second,
        }
    }
}

impl Default for TokenMeter {
    fn default() -> Self {
        Self::new()
    }
}
