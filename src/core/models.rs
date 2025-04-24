use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}

pub fn process_document(doc_id: &str) -> Result<String, CoreError> {
    // Simulate a document processing function that might fail
    if doc_id.is_empty() {
        return Err(CoreError::InvalidDocument("Empty document ID".to_string()));
    }

    if doc_id == "not_found" {
        return Err(CoreError::DocumentNotFound(doc_id.to_string()));
    }

    if doc_id == "storage_error" {
        return Err(CoreError::StorageError(
            "Failed to read document".to_string(),
        ));
    }

    Ok(format!("Processed document content for ID: {}", doc_id))
}

pub fn validate_document(content: &str) -> Result<(), CoreError> {
    // Simulate document validation
    if content.len() < 10 {
        return Err(CoreError::InvalidDocument("Document too short".to_string()));
    }

    Ok(())
}
