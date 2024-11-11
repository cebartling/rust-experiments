# Rust Axum API

A simple REST API built with Rust, using Axum web framework and PostgreSQL database.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL (running instance)
- Cargo (Rust's package manager)

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-axum-api
```

2. Set up the database:
   - Ensure PostgreSQL is running
   - The default connection string is: `postgres://postgres:postgres@localhost:5432/postgres`
   - Alternatively, set your own database URL:
```bash
export DATABASE_URL=postgres://username:password@localhost:5432/database_name
```

3. Build the project:
```bash
cargo build
```

4. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### GET /
Returns a simple hello world message.

**Response:**
```json
{
    "message": "Hello, World!"
}
```

### GET /health
Checks the health of the service and database connection.

**Response:**
```json
{
    "message": "Service is healthy"
}
```
or
```json
{
    "message": "Database connection failed"
}
```

## Project Structure

```
.
├── Cargo.toml          # Rust dependencies and project metadata
└── src
    └── main.rs         # Application entry point and route handlers
```

## Dependencies

- `axum`: Web framework
- `tokio`: Async runtime
- `sqlx`: PostgreSQL async driver
- `serde`: Serialization/deserialization
- `tower-http`: HTTP middleware
- `tracing`: Logging and instrumentation

## Development

To run in development mode with auto-reload:
```bash
cargo watch -x run
```

## Testing

Run the tests with:
```bash
cargo test
```

Test the API endpoints:
```bash
# Test root endpoint
curl http://localhost:3000/

# Test health check endpoint
curl http://localhost:3000/health
```


## License

This project is licensed under the MIT License - see the LICENSE file for details.