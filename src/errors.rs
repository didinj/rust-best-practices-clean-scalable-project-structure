// src/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyAppError {
    #[error("Invalid input: {0}")] InvalidInput(String),

    #[error("Database error: {0}")] DatabaseError(String),

    #[error("Unknown error occurred")]
    Unknown,
}
