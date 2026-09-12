use chrono::Utc;
use forge_ledger::{AiSessionRecord, BlockRecord, LedgerSummaryStats, WebhookRecord};
use serde::{Deserialize, Serialize};

/// System environment and runtime metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemMetadata {
    pub host: String,
    pub version: String,
    pub environment: String,
    pub generated_at_utc: String,
    pub uptime_seconds: u64,
}

impl Default for SystemMetadata {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            version: "0.1.0".to_string(),
            environment: "development".to_string(),
            generated_at_utc: Utc::now().to_rfc3339(),
            uptime_seconds: 0,
        }
    }
}

/// Single Source of Truth (SST) Report Document
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportDocument {
    pub metadata: SystemMetadata,
    pub summary: LedgerSummaryStats,
    pub ai_sessions: Vec<AiSessionRecord>,
    pub webhooks: Vec<WebhookRecord>,
    pub blocks: Vec<BlockRecord>,
}
