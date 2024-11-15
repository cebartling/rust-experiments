use crate::handlers::user_handlers::{
    create_user,
    delete_user,
    get_user,
    list_users,
    update_user,
};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{middleware, routing::{delete, get, post, put},  Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::Level;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::handlers::health_handlers::health_check;
use crate::models::error_response::ErrorResponse;

// Add request ID to trace spans
#[derive(Clone, Debug)]
struct RequestId(String);


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

// #[utoipa::path(
//     get,
//     path = "/health",
//     responses(
//         (status = 200, description = "Health check OK", body = Message),
//         (status = 404, description = "Health check not found", body = ErrorResponse),
//         (status = 500, description = "Internal server error", body = ErrorResponse)
//     ),
//     security(
//         ("api_key" = [])
//     ),
//     tag = "health"
// )]
// async fn health_check(State(pool): State<PgPool>) -> impl IntoResponse {
//     debug!("Starting health check");
//     let start = Instant::now();
// 
//     let result = sqlx::query("SELECT 1").execute(&pool).await;
//     let duration = start.elapsed();
// 
//     match result {
//         Ok(_) => {
//             info!(duration_ms = duration.as_millis(), "Database health check successful");
//             Json(Message {
//                 message: String::from("Service is healthy"),
//             })
//         }
//         Err(e) => {
//             error!(error = %e, duration_ms = duration.as_millis(), "Database health check failed");
//             Json(Message {
//                 message: String::from("Database connection failed"),
//             })
//         }
//     }
// }


// Generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{CreateUserRequest, User};
    use axum::http::header;
    use axum::{body::Body, http::{Request, StatusCode}, Router};
    use http_body_util::BodyExt;
    use tower::ServiceExt;


    #[tokio::test]
    async fn create_user_success() {
        let app = setup_test_app().await;
        let uuid = Uuid::new_v4();
        let email = format!("test+{uuid}@example.com");
        let create_user_request = CreateUserRequest {
            email: email.clone(),
            password: "password123".to_string(),
            full_name: "Test User".to_string(),
        };

        let request = Request::builder()
            .uri("/users")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&create_user_request).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let location = response
            .headers()
            .get(header::LOCATION)
            .expect("Location header should be present");

        assert!(location.to_str().unwrap().starts_with("/users/"));
    }

    #[tokio::test]
    async fn create_user_missing_fields() {
        let app = setup_test_app().await;
        let uuid = Uuid::new_v4();
        let email = format!("test+{uuid}@example.com");
        let body = format!(r#"{{"email": "{}"}}"#, email.clone());

        let request = Request::builder()
            .uri("/users")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(body.clone()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_user_success() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/users/1")
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email": "updated@example.com", "full_name": "Updated User"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let user: User = serde_json::from_slice(&body).unwrap();

        assert_eq!(user.email, "updated@example.com");
        assert_eq!(user.full_name, "Updated User");
    }

    #[tokio::test]
    async fn update_user_not_found() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/users/non-existent")
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email": "updated@example.com", "full_name": "Updated User"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_user_success() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/users/1")
            .method("DELETE")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_user_not_found() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .uri("/users/non-existent")
            .method("DELETE")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
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

    async fn setup_test_app() -> Router {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        create_router(pool)
    }
}