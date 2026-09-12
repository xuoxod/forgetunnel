use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detailed 5W1H execution minutiae
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetryMinutiae {
    pub duration_ps: u64,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub ttft_ms: f64,
    pub tps: f64,
    pub payload_bytes: usize,
    pub safety_tier: String,
    pub dry_run: bool,
    pub status_code: u16,
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

impl Default for TelemetryMinutiae {
    fn default() -> Self {
        Self {
            duration_ps: 0,
            prompt_tokens: 0,
            completion_tokens: 0,
            ttft_ms: 0.0,
            tps: 0.0,
            payload_bytes: 0,
            safety_tier: "Tier1".to_string(),
            dry_run: false,
            status_code: 200,
            extra: HashMap::new(),
        }
    }
}

/// 5W1H Micro-Provenance Telemetry Event Schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetryProvenance {
    pub timestamp_utc: String,
    pub who: String,
    pub from: String,
    pub to: String,
    pub what: String,
    pub how: String,
    pub minutiae: TelemetryMinutiae,
}

impl TelemetryProvenance {
    /// Create a new 5W1H provenance record with automatic UTC ISO-8601 timestamp
    pub fn new(
        who: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
        what: impl Into<String>,
        how: impl Into<String>,
        minutiae: TelemetryMinutiae,
    ) -> Self {
        Self {
            timestamp_utc: Utc::now().to_rfc3339(),
            who: who.into(),
            from: from.into(),
            to: to.into(),
            what: what.into(),
            how: how.into(),
            minutiae,
        }
    }

    /// Compute canonical SHA-256 payload digest of this provenance event
    pub fn compute_payload_digest(&self) -> String {
        use sha2::{Digest, Sha256};
        let serialized = serde_json::to_string(self).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        hex::encode(hasher.finalize())
    }
}
