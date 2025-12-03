/// Error types for CEP operations.

use thiserror::Error;

/// Errors that can occur during CEP operations.
#[derive(Error, Debug)]
pub enum CepError {
    /// Invalid timestamp format.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),

    /// Invalid hash format.
    #[error("invalid hash: expected 64 hex characters, got {0}")]
    InvalidHash(String),

    /// Invalid identifier format.
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    /// Missing required field.
    #[error("missing required field: {0}")]
    MissingField(String),

    /// Schema version mismatch.
    #[error("unsupported schema version: {0}")]
    UnsupportedVersion(String),

    /// Hash verification failed.
    #[error("hash verification failed: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Revision chain error.
    #[error("revision chain error: {0}")]
    RevisionChain(String),

    /// Schema not found or version mismatch.
    #[error("Unknown schema: {0}")]
    UnknownSchema(String),

    /// Configuration or environment error.
    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Result type for CEP operations.
pub type CepResult<T> = Result<T, CepError>;