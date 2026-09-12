use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("SQLite database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Blockchain tamper detected in ledger: {0}")]
    TamperDetected(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Lock acquisition error: {0}")]
    Lock(String),
}
