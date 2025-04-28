use std::fmt;

/// Represents domain document processing errors.
/// For demo purposes, the `fmt::Display` and `std::error::Error` traits are implented manually
#[derive(Debug)]
pub enum DomainError {
    DocumentNotFound(String),
    InvalidDocument(String),
    StorageError(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DocumentNotFound(s) => write!(f, "[Domain Display] Document not found: \"{}\"", s),
            Self::InvalidDocument(s) => write!(f, "[Domain Display] Invalid Document: \"{}\"", s),
            Self::StorageError(s) => write!(f, "[Domain Display] Storage Error:\"{}\"", s),
        }
    }
}

impl std::error::Error for DomainError {}

/// Simulate processing of a document with the given ID and returns its content.
///
/// # Arguments
/// * `doc_id` - A string slice that holds the document identifier
///
/// # Returns
/// * `Ok(String)` - A string containing the processed document content
/// * `Err(DomainError)` - An error variant when processing fails
pub fn process_document(doc_id: &str) -> Result<String, DomainError> {
    if doc_id.is_empty() {
        return Err(DomainError::InvalidDocument("Empty document ID".to_string()));
    }

    if doc_id == "not_found" {
        return Err(DomainError::DocumentNotFound(doc_id.to_string()));
    }

    if doc_id == "storage_error" {
        return Err(DomainError::StorageError(
            "Failed to read document".to_string(),
        ));
    }

    Ok(format!("Processed document content for ID: {}", doc_id))
}

/// Simulate document validation
pub fn validate_document(content: &str) -> Result<(), DomainError> {
    if content.len() < 10 {
        return Err(DomainError::InvalidDocument("Document too short".to_string()));
    }

    Ok(())
}
