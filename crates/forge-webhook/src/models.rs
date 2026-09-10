use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub payload: Vec<u8>,
    pub source_ip: String,
    pub status: u16,
}

impl WebhookRecord {
    pub fn payload_utf8_lossy(&self) -> String {
        String::from_utf8_lossy(&self.payload).to_string()
    }
}
