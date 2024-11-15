use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::PgPool;
use std::time::Instant;
use tracing::{debug, error, info};

use crate::models::message::Message;


pub async fn health_check(State(pool): State<PgPool>) -> impl IntoResponse {
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
