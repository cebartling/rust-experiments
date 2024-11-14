mod routes;
mod handlers;
mod error;
mod models;

use crate::routes::create_router;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing::{ error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn setup_logging() {
    // Get log level from environment variable or default to INFO
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string());

    // Initialize tracing subscriber with formatting and filtering
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("rust_axum_api={},tower_http=debug", log_level).into())
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
        )
        .init();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenv::dotenv().ok();
    setup_logging();

    info!("Starting application");

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    info!("Connecting to database");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Run migrations
    rust_axum_api::run_migrations(&pool).await?;

    println!("Migrations completed successfully!");

    // Build our application with routes
    let app = create_router(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Failed to bind to address"),
        app,
    )
        .await
        .expect("Failed to start server");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}, Router};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn setup_test_app() -> Router {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        create_router(pool)
    }

    #[tokio::test]
    async fn test_hello_endpoint() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app
            .oneshot(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Message = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            Message {
                message: "Hello, World!".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_health_check_endpoint() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app
            .oneshot(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Message = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            Message {
                message: "Service is healthy".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/non-existent")
            .body(Body::empty())
            .unwrap();

        let response = app
            .oneshot(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
