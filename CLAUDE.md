# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**authify-api** is a Rust-based authentication backend API built with Axum, following Clean Architecture principles. The project uses Firebase Authentication for token verification and is designed to evolve into a Twitter/X-like social platform.

## Development Commands

### Build and Run
```bash
cargo build          # Build debug version
cargo build --release # Build release version
cargo run            # Run locally
RUST_LOG=debug cargo run # Run with debug logs
```

### Testing
```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # Run tests with output
cargo test test_name          # Run specific test
```

### Code Quality
```bash
cargo check   # Check compilation
cargo clippy  # Run linter
cargo fmt     # Format code
```

## Architecture

### Clean Architecture Layers

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Module exports + AppState
├── config/              # Configuration management
├── domain/              # Business entities & repository traits
│   └── user/
├── application/         # Use cases, DTOs, services
│   ├── dto/
│   └── services/
├── infrastructure/      # External implementations
│   └── firebase/        # Firebase client + user repository
├── presentation/        # HTTP layer
│   ├── handlers/        # Request handlers
│   ├── middleware/      # Auth middleware
│   ├── extractors/      # Custom Axum extractors
│   └── routes.rs        # Router setup
└── error/               # Error types
```

### Technology Stack
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Authentication**: Firebase Auth (token verification via Google public keys)
- **Storage**: In-memory (development) - TODO: PostgreSQL/Firestore for production
- **Validation**: validator crate with derive macros
- **Error Handling**: thiserror + custom AppError type

### API Endpoints
```
GET  /health              # Health check (public)
POST /api/v1/auth/register # Create user profile (requires Firebase token)
GET  /api/v1/auth/me      # Get current user (requires Firebase token)
```

### Authentication Flow
1. Client authenticates with Firebase Auth SDK (frontend)
2. Client sends requests with `Authorization: Bearer <firebase-id-token>`
3. Backend verifies token using Google's public keys
4. Middleware injects `AuthenticatedUser` into request extensions
5. Handlers use `AuthUser` extractor to get authenticated user

## Environment Configuration

Required in `.env`:
```env
PORT=8081
RUST_LOG=info
ALLOWED_ORIGINS=https://your-domain.com
FIREBASE_PROJECT_ID=your-firebase-project-id
GOOGLE_APPLICATION_CREDENTIALS=./firebase-credentials.json
```

## Key Patterns

### Custom Extractors
- `AuthUser`: Extracts authenticated user from request extensions
- `ValidatedJson<T>`: JSON body with automatic validation

### Error Handling
All errors use `AppError` enum which implements `IntoResponse` for consistent API error responses.

### Repository Pattern
`UserRepository` trait in domain layer, implemented by `InMemoryUserRepository` (infrastructure layer).

## Deployment

Push to `main` triggers GitHub Actions:
1. `cargo test`
2. `cargo build --release`
3. Deploy to VPS via SSH/SCP
4. Restart PM2 process

The binary runs as PM2 process `authify-api` at `/opt/apps/authify/backend/`.
