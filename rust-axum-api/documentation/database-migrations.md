# Database Migration Guide for Rust/Axum Project

## Overview

This guide covers the setup and management of database migrations using SQLx with a PostgreSQL database in a Rust/Axum web application.

## Prerequisites

- Rust installed (latest stable version)
- PostgreSQL server running
- Basic understanding of SQL and Rust

## Project Setup

### 1. Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "time"] }
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15"

[build-dependencies]
sqlx-cli = "0.7"
```

### 2. Project Structure

Ensure your project has the following structure:
```
your_project/
├── Cargo.toml
├── .env
├── migrations/
│   └── 20241113000000_initial_setup.sql
├── src/
│   ├── lib.rs
│   └── main.rs
└── sqlx-data.json
```

### 3. Environment Setup

Create a `.env` file in your project root:
```
DATABASE_URL=postgres://username:password@localhost:5432/your_database_name
```

## Migration Management

### Installing SQLx CLI

```bash
cargo install sqlx-cli
```

### Creating a New Database

```bash
sqlx database create
```

### Creating Migrations

1. Generate a new migration file:
```bash
sqlx migrate add name_of_migration
```

2. This creates two files in your `migrations` directory:
   - `YYYYMMDDHHMMSS_name_of_migration.up.sql`
   - `YYYYMMDDHHMMSS_name_of_migration.down.sql`

### Writing Migrations

Example migration file (`20241113000000_initial_setup.sql`):

```sql
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create updated_at function
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create updated_at trigger for users
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();
```

### Running Migrations

#### Command Line
```bash
sqlx migrate run
```

#### In Code
```rust
pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}
```

### Reverting Migrations

To revert the last migration:
```bash
sqlx migrate revert
```

## Development Workflow

### 1. Offline Mode Support

Generate `sqlx-data.json` for offline compilation:
```bash
cargo sqlx prepare -- --lib
```

### 2. Migration Development Process

1. Create a new migration:
   ```bash
   sqlx migrate add feature_name
   ```

2. Edit the migration files in `migrations/`

3. Test the migration:
   ```bash
   sqlx migrate run
   ```

4. If needed, revert:
   ```bash
   sqlx migrate revert
   ```

### 3. Best Practices

- Always version control your migrations
- Make migrations idempotent when possible
- Include both up and down migrations
- Test migrations in a development environment first
- Back up your database before running migrations in production
- Use meaningful names for migration files

## Troubleshooting

### Common Issues

1. **Database Connection Errors**
   - Check if PostgreSQL is running
   - Verify DATABASE_URL in .env
   - Ensure database user has correct permissions

2. **Migration Conflicts**
   - Never edit existing migrations
   - Create new migrations to modify existing schema
   - Use `sqlx migrate revert` if you need to undo changes

3. **Offline Mode Issues**
   - Run `cargo sqlx prepare` after schema changes
   - Commit `sqlx-data.json` to version control

## Production Considerations

### Deployment Checklist

1. Back up the production database
2. Test migrations in staging environment
3. Schedule deployment during low-traffic periods
4. Have a rollback plan ready
5. Monitor database performance after migration

### Security

- Keep database credentials secure
- Use environment variables for sensitive data
- Limit database user permissions
- Audit migration scripts for security implications

## Additional Resources

- [SQLx Documentation](https://docs.rs/sqlx)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Rust Axum Framework](https://docs.rs/axum)
