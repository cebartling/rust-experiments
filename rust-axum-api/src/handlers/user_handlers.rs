use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Instant;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    #[serde(default = "default_limit")]
    limit: i32,
    #[serde(default)]
    offset: i32,
}

fn default_limit() -> i32 {
    10
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Creating user: {:?}", payload);

    // Check if email already exists
    if let Some(_) = User::find_by_email(&pool, &payload.email).await? {
        error!("Email already exists: {}", payload.email);
        return Err(AppError::new(
            StatusCode::CONFLICT,
            "Email already exists".to_string(),
        ));
    }

    let start = Instant::now();
    let user = User::create(&pool, payload).await?;
    let duration = start.elapsed();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        HeaderValue::from_str(&format!("/users/{}", user.id))
            .expect("Failed to create location header"),
    );
    info!(duration_ms = duration.as_millis(), "Create user successful");

    Ok((StatusCode::CREATED, headers, Json(user)))
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    debug!("Fetching user: {:?}", id);

    let start = Instant::now();
    let user = User::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))?;
    let duration = start.elapsed();
    info!(duration_ms = duration.as_millis(), "Fetch user successful");

    Ok(Json(user))
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<StatusCode, AppError> {
    debug!("Updating user: {:?}, request: {:?}", id, payload);

    let start = Instant::now();
    let _user = User::update(&pool, id, payload)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))?;
    let duration = start.elapsed();
    info!(duration_ms = duration.as_millis(), "Update user successful");

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    debug!("Deleting user: {:?}", id);

    let start = Instant::now();
    let deleted = User::delete(&pool, id).await?;
    let duration = start.elapsed();

    if deleted {
        info!(duration_ms = duration.as_millis(), "Delete user successful");
        Ok(StatusCode::NO_CONTENT)
    } else {
        error!("User not found: {:?}", id);
        Err(AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}

pub async fn list_users(
    State(pool): State<PgPool>,
    Query(params): Query<ListQueryParams>,
) -> Result<Json<Vec<User>>, AppError> {
    debug!("Fetching users: {:?}", params);

    let start = Instant::now();
    let users = User::list(&pool, params.limit.into(), params.offset.into()).await?;
    let duration = start.elapsed();
    info!(duration_ms = duration.as_millis(), "Fetch users successful");
    
    Ok(Json(users))
}
