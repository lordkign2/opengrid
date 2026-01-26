//! Unified error model for the OpenGrid core
//!
//! All public APIs return `Result<T, OpenGridError>` to provide consistent
//! error handling across the FFI boundary.

use thiserror::Error;

/// Unified error type for all OpenGrid operations
#[derive(Error, Debug)]
pub enum OpenGridError {
    /// Node creation or lifecycle error
    #[error("Node error: {0}")]
    NodeError(String),

    /// CRDT convergence or merge error
    #[error("CRDT error: {0}")]
    CrdtError(String),

    /// Storage or persistence error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Synchronization or network error
    #[error("Sync error: {0}")]
    SyncError(String),

    /// Cryptographic validation error
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Invalid input or configuration
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Internal system error
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl OpenGridError {
    /// Create a node-related error
    pub fn node(msg: impl Into<String>) -> Self {
        Self::NodeError(msg.into())
    }

    /// Create a CRDT-related error
    pub fn crdt(msg: impl Into<String>) -> Self {
        Self::CrdtError(msg.into())
    }

    /// Create a storage-related error
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::StorageError(msg.into())
    }

    /// Create a sync-related error
    pub fn sync(msg: impl Into<String>) -> Self {
        Self::SyncError(msg.into())
    }

    /// Create a crypto-related error
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::CryptoError(msg.into())
    }

    /// Create an invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }
}