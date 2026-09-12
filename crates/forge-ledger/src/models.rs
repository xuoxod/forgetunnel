use serde::{Deserialize, Serialize};

/// Persisted AI streaming session telemetry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiSessionRecord {
    pub session_id: String,
    pub model: String,
    pub prompt_preview: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub ttft_ms: f64,
    pub tps: f64,
    pub duration_ms: f64,
    pub status: String,
    pub created_at: String,
}

/// Persisted Webhook interception & replay record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookRecord {
    pub event_id: String,
    pub provider: String,
    pub event_type: String,
    pub signature_header: String,
    pub signature_valid: bool,
    pub payload_bytes: usize,
    pub payload_body: String,
    pub replay_count: u32,
    pub status_code: u16,
    pub created_at: String,
}

/// Persisted rolling blockchain block record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockRecord {
    pub index: u64,
    pub block_hash: String,
    pub prev_hash: String,
    pub payload_digest: String,
    pub who: String,
    pub source_addr: String,
    pub dest_addr: String,
    pub action: String,
    pub transport: String,
    pub safety_tier: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub ttft_ms: f64,
    pub tps: f64,
    pub duration_ps: u64,
    pub timestamp_utc: String,
    pub raw_json: String,
}

/// Summary statistics aggregate for reporting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LedgerSummaryStats {
    pub total_ai_sessions: u64,
    pub total_prompt_tokens: u64,
    pub total_completion_tokens: u64,
    pub avg_ttft_ms: f64,
    pub avg_tps: f64,
    pub total_webhooks: u64,
    pub valid_webhooks: u64,
    pub total_replays: u64,
    pub total_blocks: u64,
}
