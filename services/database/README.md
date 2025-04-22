# Lyn User Data Service (database)

This Rust crate implements the User Data Service for the Lyn AI Assistant, providing isolated, encrypted storage for user data using SeaORM and PostgreSQL.

## Features
- Isolated service for user data (settings, conversations, audit logs)
- SeaORM integration for async, type-safe database access
- PostgreSQL as the backing store
- Encryption at rest for sensitive fields (planned)
- Audit logging for all data access
- Modular API for CRUD operations on user data and settings

## Architecture
- Models defined in `src/entities.rs`
- Main entry point in `src/main.rs` (connects to PostgreSQL, sets up runtime)
- Service logic and API handlers to be implemented (see planning docs)

## Integration
- Designed to run as a separate service in the Lyn backend architecture
- Exposes endpoints for frontend and other backend services to interact with user data
- All access is logged for auditing and privacy compliance

## Getting Started
1. Set the `DATABASE_URL` environment variable to your PostgreSQL connection string.
2. Run migrations (to be implemented).
3. Start the service with `cargo run`.

## Planned Improvements
- Implement encryption at rest for sensitive fields
- Add REST/gRPC API for CRUD operations
- Integrate audit logging for all data access
- Add migrations and schema management

See the main project documentation for more details on architecture and integration.