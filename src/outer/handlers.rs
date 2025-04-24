use anyhow::{self, Context};
use thiserror::Error;
use crate::intermediate::services::{self, ServiceError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

pub fn api_process_document(doc_id: &str) -> Result<(), ApiError> {
    // Validate input
    if doc_id.trim().is_empty() {
        return Err(ApiError::BadRequest("Document ID cannot be empty".to_string()));
    }

    // Call service layer
    let result = services::handle_document(doc_id)
        .map_err(|err| {
            match err {
                ServiceError::ProcessingError(msg) => {
                    ApiError::BadRequest(format!("Could not process document: {}", msg))
                },
                ServiceError::ValidationError(msg) => {
                    ApiError::BadRequest(msg)
                },
                // For other errors, use internal error or automatic conversion
                _ => ApiError::InternalError("An unexpected error occurred".to_string()),
            }
        })?;
        // .context(format!("API request failed for document {}", doc_id))?;

    println!("API successfully processed document: {}", result);
    Ok(())
}

pub fn api_create_document(doc_id: &str, content: &str) -> Result<(), ApiError> {
    // Validate inputs
    if doc_id.trim().is_empty() {
        return Err(ApiError::BadRequest("Document ID cannot be empty".to_string()));
    }

    if content.trim().is_empty() {
        return Err(ApiError::BadRequest("Document content cannot be empty".to_string()));
    }

    // Call service layer
    services::validate_and_process(doc_id, content)
        .map_err(|err| {
            match err {
                ServiceError::ValidationError(msg) => {
                    ApiError::BadRequest(msg)
                },
                ServiceError::ProcessingError(msg) => {
                    ApiError::BadRequest(msg)
                },
                ServiceError::CoreError(core_err) => {
                    ApiError::InternalError(format!("Core system error: {}", core_err))
                },
            }
        })?;
        //.context("Document creation failed")?;

    println!("API successfully created document with ID: {}", doc_id);
    Ok(())
}
