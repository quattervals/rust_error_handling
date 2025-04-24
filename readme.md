# Onion Architecture Error Handling Example

This project demonstrates error handling in a Rust application structured using the onion architecture pattern. The application is organized into three distinct layers:

1. **Core Layer (Inner)**: Contains the core business logic and fundamental error types
2. **Intermediate Layer**: Service layer that uses the core and provides additional context to errors
3. **Outer Layer**: Consumer/API layer that interacts with the outside world

## Project Structure

```
.
├── Cargo.toml
└── src
    ├── core
    │   ├── models.rs    # Domain models + Core error types
    │   └── mod.rs
    ├── intermediate
    │   ├── services.rs  # Service implementations + Service error types
    │   └── mod.rs
    ├── outer
    │   ├── handlers.rs  # API handlers + API error types
    │   └── mod.rs
    └── main.rs          # Application entry point
```

## Error Handling Approach

This project demonstrates several error handling techniques:

1. **Type-safe error enums** using `thiserror`
2. **Error context and chains** using `anyhow`
3. **Error mapping** between layers
4. **Automatic conversion** using the `#[from]` attribute
5. **Pattern matching** for specific error handling

## Design Philosophy

Each layer defines its own error types directly within its main module file:

- **Core Layer**: `CoreError` in models.rs
- **Intermediate Layer**: `ServiceError` in services.rs
- **Outer Layer**: `ApiError` in handlers.rs

This approach keeps error definitions close to the code that uses them, making the relationship between errors and their respective modules clearer.

## Running the Project

To run the example:

```bash
cargo run
```

This will execute several test cases that demonstrate how errors propagate through the layers and how they are handled at each level.
