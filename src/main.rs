mod api;
mod domain;
mod usecases;

fn main() {
    println!("=== Testing Error Handling in Onion Architecture ===\n");

    println!("Valid document");
    match api::api_process_document("valid_doc") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => {
            println!("❌ Error: {:#?}", e);
        }
    };
    println!();

    println!("Document not found");
    match api::api_process_document("not_found") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!("Empty document ID");
    match api::api_process_document("") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!(" Storage error");
    match api::api_process_document("storage_error") {
        Ok(_) => println!("✅ Success: Document processed successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!("Creating document with valid content");
    match api::api_create_document("new_doc", "This is a valid document with enough content") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!("Creating document with invalid content");
    match api::api_create_document("new_doc", "Too short") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!("Creating document with empty ID");
    match api::api_create_document("", "Too short") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();

    println!("Creating document with empty content");
    match api::api_create_document("sane_id", "") {
        Ok(_) => println!("✅ Success: Document created successfully"),
        Err(e) => println!("❌ Error: {:#?}", e),
    }
    println!();
}
