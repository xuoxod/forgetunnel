use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForgeError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML serialization/deserialization error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Safety violation: tier {tier:?}, action '{action}': {reason}")]
    SafetyViolation {
        tier: String,
        action: String,
        reason: String,
    },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Cryptographic signature verification failed")]
    SignatureInvalid,

    #[error("Network/Stream error: {0}")]
    Network(String),

    #[error("Upstream service unavailable: {0}")]
    UpstreamUnavailable(String),
}

pub type Result<T> = std::result::Result<T, ForgeError>;
