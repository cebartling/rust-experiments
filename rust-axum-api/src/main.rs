use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
};
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
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, Level, };
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
    ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

// Add request ID to trace spans
#[derive(Clone, Debug)]
struct RequestId(String);

async fn trace_request_id(request: Request<Body>, next: Next) -> impl IntoResponse {
    let request_id = RequestId(Uuid::new_v4().to_string());

    // Store request_id in request extensions
    let mut request = request;
    request.extensions_mut().insert(request_id.clone());

    let span = tracing::span!(
        Level::INFO,
        "request",
        request_id = %request_id.0
    );

    let _guard = span.enter();
    next.run(request).await
}

#[derive(Serialize, Debug, Deserialize, PartialEq, ToSchema)]
struct Message {
    message: String,
}

// Example error response
#[derive(ToSchema)]
#[allow(dead_code)]
struct ErrorResponse {
    #[schema(example = "Invalid input provided")]
    message: String,
    #[schema(example = 400)]
    code: i32,
}

// Security modifier to add API key authentication
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
        );
    }
}

// Generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
    ),
    components(
        schemas(ErrorResponse)
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "My API",
        version = "1.0.0",
        description = "API documentation for my Rust/Axum service",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;


async fn hello_handler() -> Json<Message> {
    info!("Handling hello request");
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check OK", body = Message),
        (status = 404, description = "Health check not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "health"
)]
async fn health_check(State(pool): State<PgPool>) -> impl IntoResponse {
    debug!("Starting health check");
    let start = Instant::now();

    let result = sqlx::query("SELECT 1").execute(&pool).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => {
            info!(duration_ms = duration.as_millis(), "Database health check successful");
            Json(Message {
                message: String::from("Service is healthy"),
            })
        }
        Err(e) => {
            error!(error = %e, duration_ms = duration.as_millis(), "Database health check failed");
            Json(Message {
                message: String::from("Database connection failed"),
            })
        }
    }
}

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(hello_handler))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(trace_request_id))
        .with_state(pool)
}

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
async fn main() {
    setup_logging();

    info!("Starting application");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    info!("Connecting to database");
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        },
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Build our application with routes
    let app = create_router(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Failed to bind to address"),
        app
    )
        .await
        .expect("Failed to start server");
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

    async fn setup_test_app() -> Router {
        let database_url = std::env::var("DATABASE_URL")
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
