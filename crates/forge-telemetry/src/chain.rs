use crate::error::TelemetryError;
use crate::provenance::TelemetryProvenance;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// An immutable block in the tamper-evident SHA-256 blockchain ledger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp_utc: String,
    pub prev_hash: String,
    pub payload_digest: String,
    pub entry: TelemetryProvenance,
    pub block_hash: String,
}

impl Block {
    /// Compute the cryptographic block hash over index, timestamp, prev_hash, and payload_digest
    pub fn calculate_hash(
        index: u64,
        timestamp_utc: &str,
        prev_hash: &str,
        payload_digest: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp_utc.as_bytes());
        hasher.update(prev_hash.as_bytes());
        hasher.update(payload_digest.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Verify this block's hash and internal payload digest integrity
    pub fn verify(&self) -> Result<(), TelemetryError> {
        let expected_payload = self.entry.compute_payload_digest();
        if self.payload_digest != expected_payload {
            return Err(TelemetryError::TamperDetected {
                index: self.index,
                calculated: expected_payload,
                expected: self.payload_digest.clone(),
            });
        }

        let calculated_block_hash = Self::calculate_hash(
            self.index,
            &self.timestamp_utc,
            &self.prev_hash,
            &self.payload_digest,
        );

        if self.block_hash != calculated_block_hash {
            return Err(TelemetryError::TamperDetected {
                index: self.index,
                calculated: calculated_block_hash,
                expected: self.block_hash.clone(),
            });
        }

        Ok(())
    }
}

/// Rolling SHA-256 Blockchain Hash-Chain Ledger Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChain {
    genesis_hash: String,
    blocks: Vec<Block>,
}

impl HashChain {
    /// Initialize a new blockchain ledger seeded with a cryptographic genesis block
    pub fn new(seed: &str) -> Self {
        let _genesis_time = Utc::now().to_rfc3339();
        let mut hasher = Sha256::new();
        hasher.update(b"FORGETUNNEL_SOVEREIGN_GENESIS_V1:");
        hasher.update(seed.as_bytes());
        let genesis_hash = hex::encode(hasher.finalize());

        Self {
            genesis_hash,
            blocks: Vec::new(),
        }
    }

    /// Append a new 5W1H provenance record to the blockchain
    pub fn append(&mut self, entry: TelemetryProvenance) -> Block {
        let index = self.blocks.len() as u64;
        let prev_hash = if let Some(last) = self.blocks.last() {
            last.block_hash.clone()
        } else {
            self.genesis_hash.clone()
        };

        let timestamp_utc = Utc::now().to_rfc3339();
        let payload_digest = entry.compute_payload_digest();
        let block_hash = Block::calculate_hash(index, &timestamp_utc, &prev_hash, &payload_digest);

        let block = Block {
            index,
            timestamp_utc,
            prev_hash,
            payload_digest,
            entry,
            block_hash,
        };

        self.blocks.push(block.clone());
        block
    }

    /// Verify total cryptographic chain integrity from genesis to current block
    pub fn verify_integrity(&self) -> Result<(), TelemetryError> {
        let mut expected_prev_hash = self.genesis_hash.as_str();

        for (i, block) in self.blocks.iter().enumerate() {
            let index = i as u64;
            if block.index != index {
                return Err(TelemetryError::InvalidSequence {
                    expected: index,
                    actual: block.index,
                });
            }

            if block.prev_hash != expected_prev_hash {
                return Err(TelemetryError::InvalidPrevHash {
                    index,
                    actual: block.prev_hash.clone(),
                    expected: expected_prev_hash.to_string(),
                });
            }

            block.verify()?;
            expected_prev_hash = &block.block_hash;
        }

        Ok(())
    }

    /// Get slice of all confirmed blocks
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Get current rolling ledger block count
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get hash of the latest block, or genesis hash if empty
    pub fn current_hash(&self) -> &str {
        self.blocks
            .last()
            .map(|b| b.block_hash.as_str())
            .unwrap_or(self.genesis_hash.as_str())
    }

    /// Get the genesis seed hash
    pub fn genesis_hash(&self) -> &str {
        &self.genesis_hash
    }
}
