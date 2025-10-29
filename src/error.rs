use thiserror::Error; // 1. Use thiserror::Erroruse std::io;
use std::io;

#[derive(Error, Debug)] // 2. Change derive to Error
pub enum KvsError {
    /// IO error.
    #[error("{0}")] // 3. Use #[error] for the display message
    Io(#[from] io::Error), // 4. #[from] auto-generates the From impl

    /// Serialization or deserialization error.
    #[error("{0}")]
    Serde(#[from] serde_json::Error), // 5. This also auto-generates From

    /// Removing non-existent key error.
    #[error("Key not found")]
    KeyNotFound,

    /// Unexpected command type error.
    #[error("Unexpected command type")]
    UnexpectedCommandType,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;