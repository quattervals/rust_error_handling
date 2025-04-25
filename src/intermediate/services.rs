use crate::core::{self, models::CoreError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Core error: {0}")]
    CoreError(#[from] CoreError),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

//// Handles document processing through the core service layer.
///
/// The mapping for the CoreErrors to ServiceError::CoreError
/// is covered with `CoreError(#[from] CoreError)`
///
/// # Returns
/// * `Ok(String)` - Successfully processed document content with service layer prefix
/// * `Err(ServiceError::CoreError)`

pub fn handle_document(doc_id: &str) -> Result<String, ServiceError> {
    let content = core::models::process_document(doc_id)?;
    Ok(format!("Service processed: {}", content))
}

/// Error Forwarding Example
/// We could forward all CoreErrors::* to ServiceError::CoreError()
/// using `core::models::validate_document(content)?;`
///
/// But we chose to do some handling manually to aggregate some more context.
/// To get and pass on the full display of the inner error,
/// we need to extract the full error message before matching the errors.
///
///
/// # Errors
///
/// Returns ServiceError

pub fn validate_and_process(doc_id: &str, content: &str) -> Result<String, ServiceError> {
    core::models::validate_document(content).map_err(|err| {
        let full_error_message = err.to_string();
        match err {
            CoreError::InvalidDocument(reason) => ServiceError::ValidationError(format!(
                "Document validation failed: {} -- {}",
                reason, full_error_message
            )),
            _ => ServiceError::CoreError(err),
        }
    })?;

    handle_document(doc_id)
}
