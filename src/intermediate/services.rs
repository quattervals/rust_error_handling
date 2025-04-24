use crate::core::{self, models::CoreError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Core error: {0}")]
    CoreError(#[from] CoreError),

    #[error("Document processing failed: {0}")]
    ProcessingError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub fn handle_document(doc_id: &str) -> Result<String, ServiceError> {
    // Call the core function and map errors with context
    let content = core::models::process_document(doc_id)
        .map_err(|err| {
            match err {
                CoreError::DocumentNotFound(id) => ServiceError::ProcessingError(format!(
                    "Document {} could not be found in the system",
                    id
                )),
                // For other errors, use the automatic conversion via #[from]
                _ => err.into(),
            }
        })?;


    Ok(format!("Service processed: {}", content))
}

pub fn validate_and_process(doc_id: &str, content: &str) -> Result<String, ServiceError> {

    core::models::validate_document(content).map_err(|err| match err {
        CoreError::InvalidDocument(reason) => {
            ServiceError::ValidationError(format!("Document validation failed: {}", reason))
        }
        _ => err.into(),
    })?;

    handle_document(doc_id)
}
