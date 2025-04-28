use anyhow::{self, Context};
use thiserror::Error;

use crate::usecases::usecases::{self, UseCaseError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Use case error: {0}")]
    UseCaseError(#[from] UseCaseError),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

pub fn api_process_document(doc_id: &str) -> anyhow::Result<()> {
    if doc_id.trim().is_empty() {
        return Err(anyhow::anyhow!(ApiError::BadRequest(
            "Document ID cannot be empty".to_string()
        )));
    }

    let result = usecases::handle_document(doc_id).map_err(|err| {
        let api_err = match err {
            UseCaseError::ValidationError(msg) => ApiError::BadRequest(msg),
            _ => ApiError::InternalError(err.to_string()),
        };
        anyhow::anyhow!(api_err).context(format!("Failed processing document {}", doc_id))
    })?;

    println!("API successfully processed document: {}", result);
    Ok(())
}

pub fn api_create_document(doc_id: &str, content: &str) -> anyhow::Result<()> {
    if doc_id.trim().is_empty() {
        return Err(anyhow::anyhow!(ApiError::BadRequest(
            "Document ID cannot be empty".to_string()
        ),))
        .context("Context about doc_id");
    }

    if content.trim().is_empty() {
        return Err(anyhow::anyhow!(ApiError::BadRequest(
            "Document content cannot be empty".to_string(),
        )))
        .context(format!(
            "This content with id: \"{}\" is not good.\n",
            doc_id.trim()
        ));
    }

    usecases::validate_and_process(doc_id, content).map_err(|err| {
        let api_err = match err {
            UseCaseError::ValidationError(msg) => ApiError::BadRequest(msg),
            UseCaseError::DomainError(core_err) => {
                ApiError::InternalError(format!("domain system error: {}", core_err))
            }
        };
        anyhow::anyhow!(api_err)
    })?;

    Ok(())
}
