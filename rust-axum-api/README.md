# Rust Axum API

A simple REST API built with Rust, using Axum web framework and PostgreSQL database.

## Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- Cargo (Rust's package manager)
- SQLx CLI (for database migrations) - `cargo install sqlx-cli`

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-axum-api
```

2. Start the database:
```bash
docker-compose up -d
```

3. Migrate the database:
```bash
sqlx migrate run
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

## Database Configuration

The application uses PostgreSQL with the following default configuration:
- Host: `localhost`
- Port: `5432`
- User: `postgres`
- Password: `postgres`
- Database: `postgres`

You can override these settings by setting the `DATABASE_URL` environment variable:
```bash
export DATABASE_URL=postgres://username:password@localhost:5432/database_name
```

## Docker Commands

Start the database:
```bash
docker-compose up -d
```

Stop the database:
```bash
docker-compose down
```

View database logs:
```bash
docker-compose logs postgres
```

Reset database (removes all data):
```bash
docker-compose down -v
docker-compose up -d
```

## API Endpoints

- [OpenAPI/Swagger documentation How-To](./documentation/open-api.md)
- [Live OpenAPI/Swagger docs](http://localhost:3000/swagger-ui) 

## Project Structure

```
.
├── Cargo.toml          # Rust dependencies and project metadata
├── docker-compose.yml  # Docker configuration
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

