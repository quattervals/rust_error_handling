mod core;
mod intermediate;
mod outer;

use anyhow::Result;
use outer::handlers::ApiError;

fn main() -> Result<()> {
    println!("=== Testing Error Handling in Onion Architecture ===\n");

    // Test Case 1: Valid document
    println!("Test Case 1: Valid document");
    match outer::handlers::api_process_document("valid_doc") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    // Test Case 2: Document not found
    println!("Test Case 2: Document not found");
    match outer::handlers::api_process_document("not_found") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    // Test Case 3: Empty document ID
    println!("Test Case 3: Empty document ID");
    match outer::handlers::api_process_document("") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    // Test Case 4: Storage error
    println!("Test Case 4: Storage error");
    match outer::handlers::api_process_document("storage_error") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    // Test Case 5: Creating a document with valid content
    println!("Test Case 5: Creating document with valid content");
    match outer::handlers::api_create_document("new_doc", "This is a valid document with enough content") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    // Test Case 6: Creating a document with invalid content
    println!("Test Case 6: Creating document with invalid content");
    match outer::handlers::api_create_document("new_doc", "Too short") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => print_error(&e, "❌ Error"),
    }
    println!();

    Ok(())
}

fn print_error(error: &ApiError, prefix: &str) {
    println!("{}: {}", prefix, error);

    // If we want to print the full error chain:
    if let Some(source) = std::error::Error::source(error) {
        println!("Caused by: {}", source);
        let mut current = source;
        while let Some(next) = std::error::Error::source(current) {
            println!("Caused by: {}", next);
            current = next;
        }
    }
}
