use crate::models::WebhookRecord;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct WebhookInspector {
    capacity: usize,
    buffer: Arc<RwLock<VecDeque<WebhookRecord>>>,
}

impl WebhookInspector {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(capacity))),
        }
    }

    pub fn record(&self, entry: WebhookRecord) {
        let mut buf = self.buffer.write().unwrap();
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(entry);
    }

    pub fn list_recent(&self, limit: usize) -> Vec<WebhookRecord> {
        let buf = self.buffer.read().unwrap();
        buf.iter().rev().take(limit).cloned().collect()
    }

    pub fn get(&self, id: &str) -> Option<WebhookRecord> {
        let buf = self.buffer.read().unwrap();
        buf.iter().find(|r| r.id == id).cloned()
    }

    pub fn count(&self) -> usize {
        let buf = self.buffer.read().unwrap();
        buf.len()
    }

    pub fn clear(&self) {
        let mut buf = self.buffer.write().unwrap();
        buf.clear();
    }

    pub fn verify_hmac_signature(secret: &str, payload: &[u8], signature_hex: &str) -> bool {
        let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        mac.update(payload);
        let expected = hex::encode(mac.finalize().into_bytes());
        expected.eq_ignore_ascii_case(signature_hex)
    }
}
