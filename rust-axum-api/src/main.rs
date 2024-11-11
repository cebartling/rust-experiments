// src/main.rs
use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
struct Message {
    message: String,
}

async fn hello_handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

async fn health_check(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => Json(Message {
            message: String::from("Service is healthy"),
        }),
        Err(_) => Json(Message {
            message: String::from("Database connection failed"),
        }),
    }
}

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(hello_handler))
        .route("/health", get(health_check))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Build our application with routes
    let app = create_router(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app,
    )
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello_endpoint() {
        // Create test database pool
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        // Build our application with routes
        let app = create_router(pool);

        // Create request
        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        // Get response
        let response = app
            .oneshot(request)
            .await
            .unwrap();

        // Assert status code
        assert_eq!(response.status(), StatusCode::OK);

        // Get response body
        let body = response.into_body().collect().await.unwrap().to_bytes();

        // Parse response body as JSON and verify the content
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
        // Create test database pool
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        // Build our application with routes
        let app = create_router(pool);

        // Create request
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        // Get response
        let response = app
            .oneshot(request)
            .await
            .unwrap();

        // Assert status code
        assert_eq!(response.status(), StatusCode::OK);

        // Get response body
        let body = response.into_body().collect().await.unwrap().to_bytes();

        // Parse response body as JSON and verify the content
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
        // Create test database pool
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        // Build our application with routes
        let app = create_router(pool);

        // Create request to non-existent endpoint
        let request = Request::builder()
            .uri("/non-existent")
            .body(Body::empty())
            .unwrap();

        // Get response
        let response = app
            .oneshot(request)
            .await
            .unwrap();

        // Assert status code
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
