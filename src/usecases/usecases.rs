use thiserror::Error;

use crate::domain::domain::{self, DomainError};

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("domain error: {0}")]
    DomainError(#[from] DomainError),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Handles document processing through the domain layer.
///
/// The mapping for the DomainErrors to UseCaseError::DomainError
/// is covered with `DomainError(#[from] DomainError)`
///
/// # Returns
/// * `Ok(String)` - Successfully processed document content with service layer prefix
/// * `Err(UseCaseError::DomainError)`

pub fn handle_document(doc_id: &str) -> Result<String, UseCaseError> {
    let content = domain::process_document(doc_id)?;
    Ok(format!("Service processed: {}", content))
}

/// Error Forwarding Example
///
/// We could forward all DomainErrors::* to UseCaseError::DomainError()
/// using `domain::domain::validate_document(content)?;`
///
/// But we chose to do some handling manually to aggregate some more context.
/// To get and pass on the full display of the inner error,
/// we need to extract the full error message before matching the errors.
///
///
/// # Errors
///
/// Returns UseCaseError

pub fn validate_and_process(doc_id: &str, content: &str) -> Result<String, UseCaseError> {
    domain::validate_document(content).map_err(|err| {
        let full_error_message = err.to_string();
        match err {
            DomainError::InvalidDocument(reason) => UseCaseError::ValidationError(format!(
                "Document validation failed: {} -- {}",
                reason, full_error_message
            )),
            _ => UseCaseError::DomainError(err),
        }
    })?;

    handle_document(doc_id)
}
