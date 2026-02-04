use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Node identity error: {0}")]
    IdentityError(String),

    #[error("Storage failure: {0}")]
    StorageError(String),

    #[error("Sync protocol violation: {0}")]
    SyncError(String),

    #[error("CRDT merge conflict or invalid state: {0}")]
    CrdtError(String),

    #[error("Crypto verification failed")]
    CryptoError,

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;