# Onion Architecture Error Handling Example

This project demonstrates error handling in a Rust application structured using an onion-like architecture pattern.
The application is organized into three layers:

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
    └── main.rs          # User application
```

## Error Handling Approach

This project demonstrates several error handling techniques:

1. **Type-safe error enums** using `thiserror`
2. **Error context and chains** using `anyhow`
3. **Error mapping** between layers
4. **Automatic conversion** using the `#[from]` attribute
5. **Pattern matching** for specific error handling

## Design Philosophy

On the inner layers, enum errors are passed up the call chain. These errors are handled internally when possible.

On the boundary between `handlers` and `main.rs`, `anyhow` error types are used. Only non-recoverable errors are passed to the user/main. With `anyhow`-style errors, the users just get an error and does not have to deal with specific error types.

Each layer defines its own error types directly within its main module file:


- **Core Layer**: `CoreError` in models.rs
  - For demo purposes, the display trait in this module is implemented by hand
- **Intermediate Layer**: `ServiceError` in services.rs
  - The error trait implementations are handled with the `thiserror` crate
- **Outer Layer**: `ApiError` in handlers.rs

This approach keeps error definitions close to the code that uses them, making the relationship between errors and their respective modules clearer.
