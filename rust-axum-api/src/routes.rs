use crate::handlers::user_handlers::{
    create_user,
    delete_user,
    get_user,
    list_users,
    update_user,
};
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{middleware, routing::{delete, get, post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, Level};
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


pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(trace_request_id))
        .with_state(pool)
}
