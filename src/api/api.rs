use crate::usecases::usecases::{self, UseCaseError};
use error_handling::StringFrom;

#[derive(Debug, StringFrom)]
#[allow(dead_code, reason = "compiler thinks the Strings are unused")]
pub enum ApiError {
    #[stringfrom("use case error")]
    UseCaseError(String, #[from] UseCaseError),
    BadRequest(String),
}

// impl From<

pub fn api_process_document(doc_id: &str) -> Result<(), ApiError> {
    if doc_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Document ID cannot be empty".to_string(),
        ));
    }

    let result = usecases::handle_document(doc_id)?;

    println!("API successfully processed document: {}", result);
    Ok(())
}

pub fn api_create_document(doc_id: &str, content: &str) -> Result<(), ApiError> {
    if doc_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Document ID cannot be empty".to_string(),
        ));
    }

    if content.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Document content cannot be empty".to_string(),
        ));
    }

    usecases::validate_and_process(doc_id, content)?;

    Ok(())
}
