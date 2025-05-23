# Axum SQS Example

A Rust web application built with Axum that demonstrates various web development patterns including authentication, request tracing, and API endpoint implementations.

## Features

- **Authentication System**
  - JWT-based authentication
  - Protected routes with middleware
  - Token validation and claims processing

- **API Endpoints**
  - User management routes
  - Protected endpoints
  - Various request handlers (JSON, string, bytes, etc.)
  - Query parameter handling
  - Header extraction

- **Middleware**
  - Request tracing
  - Authentication middleware
  - Error handling

- **Testing**
  - Integration tests
  - Unit tests
  - Test server setup

## Project Structure

```
src/
├── lib.rs                 # Library crate entry point
├── main.rs               # Binary crate entry point
├── lib/
│   ├── app_state.rs      # Application state management
│   ├── auth_claim.rs     # JWT authentication and claims
│   ├── auth_claim_mid.rs # Authentication middleware
│   ├── backend_server.rs # Server setup and configuration
│   ├── my_extractors.rs  # Custom request extractors
│   ├── my_math.rs        # Example math functions
│   ├── protected_router.rs # Protected route handlers
│   └── users_router.rs   # User management routes
tests/
└── integration_tests.rs  # Integration test suite
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Environment Variables

Create a `.env` file in the project root with the following variables:

```env
HOST=127.0.0.1
PORT=3000
RUST_LOG=info
JWT_SECRET=your_jwt_secret_here
```

### Running the Application

```bash
cargo run
```

The server will start on the configured host and port (default: http://127.0.0.1:3000).

### Running Tests

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

## API Endpoints

### Authentication

- `POST /authorization`
  - Authenticates users and returns JWT token
  - Request body: `{ "client_id": "foo", "client_secret": "bar" }`

### Protected Routes

- `POST /protected`
  - Requires valid JWT token in Authorization header
  - Returns protected content

- `POST /protected/norm`
  - Protected endpoint that accepts JSON input
  - Requires valid JWT token

### Other Endpoints

- `GET /` - Hello World endpoint
- `GET /string-handler` - String response handler
- `POST /json` - JSON request handler
- `POST /echo` - Echo bytes handler
- `GET /headers` - Header extraction example
- `POST /input-string` - String input handler
- `POST /sample-request` - Sample request handler

## Development

### Adding New Routes

1. Create a new router module in `src/lib/`
2. Define your routes and handlers
3. Add the router to `backend_server.rs`

### Adding Tests

1. Add new test cases to `tests/integration_tests.rs`
2. Use the `spawn_test_server()` helper function
3. Run tests with `cargo test`

## Error Handling

The application includes comprehensive error handling:
- Authentication errors
- Invalid request handling
- Not found responses
- Bad request responses

## Logging

The application uses `tracing` for logging:
- Request tracing
- Response logging
- Error logging
- Configurable log levels via `RUST_LOG`

## License

This project is open source and available under the MIT License.