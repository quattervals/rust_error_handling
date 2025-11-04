use anyhow::{self, Context};

use crate::usecases::usecases::{self};

pub fn api_process_document(doc_id: &str) -> anyhow::Result<()> {
    if doc_id.trim().is_empty() {
        return Err(anyhow::anyhow!("Document ID cannot be empty"));
    }

    let result: String = usecases::handle_document(doc_id).map_err(|err| {
        anyhow::anyhow!(err).context(format!("Failed processing document {}", doc_id))
    })?;

    println!("API successfully processed document: {}", result);
    Ok(())
}

pub fn api_create_document(doc_id: &str, content: &str) -> anyhow::Result<()> {
    if doc_id.trim().is_empty() {
        return Err(anyhow::anyhow!("Document ID cannot be empty"));
    }

    if content.trim().is_empty() {
        return Err(anyhow::anyhow!("Document content cannot be empty")).context(format!(
            "This content with id: '{}' is not good.\n",
            doc_id.trim()
        ));
    }

    usecases::validate_and_process(doc_id, content).map_err(|err| anyhow::anyhow!(err))?;

    Ok(())
}
