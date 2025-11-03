use crate::usecases::usecases::{self, UseCaseError};


#[derive(Debug)]
#[allow(dead_code, reason="compiler thinks the Strings are unused")]
pub enum ApiError {
    UseCaseError(String),
    BadRequest(String),
}


/// If we didn't allow dead code, we would have to have this message() impl.
/// This would be verbose and error prone since we have to list/match all members
// impl ApiError {
//     pub fn message(&self) -> &str {
//         match self {
//             ApiError::UseCaseError(msg) => msg,
//             ApiError::BadRequest(msg) => msg,
//             ApiError::InternalError(msg) => msg,
//         }
//     }
// }

impl From<UseCaseError> for ApiError {
    fn from(err: UseCaseError) -> Self {
        ApiError::UseCaseError(format!("UseCaseError String Representation '{0}'", err))
    }
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
