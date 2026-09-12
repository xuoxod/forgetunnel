use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Tamper detected at block index {index}: calculated hash {calculated} != expected {expected}")]
    TamperDetected {
        index: u64,
        calculated: String,
        expected: String,
    },

    #[error("Invalid previous hash at block index {index}: block prev_hash {actual} != predecessor hash {expected}")]
    InvalidPrevHash {
        index: u64,
        actual: String,
        expected: String,
    },

    #[error("Invalid block sequence: expected index {expected}, found {actual}")]
    InvalidSequence {
        expected: u64,
        actual: u64,
    },

    #[error("Genesis block hash mismatch")]
    InvalidGenesis,
}
