use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::error::{ForgeError, Result};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TunnelType {
    Ollama,
    OpenAI,
    Webhook,
    Http,
    TcpRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PacketFlag {
    Syn,
    Data,
    Ack,
    Fin,
    Rst,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgePacket {
    pub packet_id: String,
    pub endpoint_id: String,
    pub stream_id: String,
    pub sequence: u64,
    pub flag: PacketFlag,
    pub payload: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

impl ForgePacket {
    pub fn new(
        endpoint_id: &str,
        stream_id: &str,
        sequence: u64,
        flag: PacketFlag,
        payload: Vec<u8>,
        secret: &str,
    ) -> Self {
        let timestamp = Utc::now();
        let packet_id = format!("pkt-{}-{}", stream_id, sequence);
        let signature = Self::compute_signature(secret, &packet_id, endpoint_id, stream_id, sequence, &payload, &timestamp);

        Self {
            packet_id,
            endpoint_id: endpoint_id.to_string(),
            stream_id: stream_id.to_string(),
            sequence,
            flag,
            payload,
            timestamp,
            signature,
        }
    }

    pub fn compute_signature(
        secret: &str,
        packet_id: &str,
        endpoint_id: &str,
        stream_id: &str,
        sequence: u64,
        payload: &[u8],
        timestamp: &DateTime<Utc>,
    ) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .unwrap_or_else(|_| HmacSha256::new_from_slice(b"default-fallback-secret-key-32b").unwrap());

        mac.update(packet_id.as_bytes());
        mac.update(endpoint_id.as_bytes());
        mac.update(stream_id.as_bytes());
        mac.update(&sequence.to_be_bytes());
        mac.update(payload);
        mac.update(timestamp.to_rfc3339().as_bytes());

        hex::encode(mac.finalize().into_bytes())
    }

    pub fn verify_signature(&self, secret: &str) -> Result<()> {
        let expected = Self::compute_signature(
            secret,
            &self.packet_id,
            &self.endpoint_id,
            &self.stream_id,
            self.sequence,
            &self.payload,
            &self.timestamp,
        );

        if self.signature == expected {
            Ok(())
        } else {
            Err(ForgeError::SignatureInvalid)
        }
    }

    pub fn sha256_payload_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.payload);
        hex::encode(hasher.finalize())
    }
}
