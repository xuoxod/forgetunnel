use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Ledger storage error: {0}")]
    Ledger(#[from] forge_ledger::LedgerError),

    #[error("Format rendering error: {0}")]
    Rendering(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
