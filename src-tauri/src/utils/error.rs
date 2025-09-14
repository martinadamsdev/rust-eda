use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Thread error: {0}")]
    ThreadError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

// AppError already implements Display trait via thiserror
// No need for custom to_string implementation