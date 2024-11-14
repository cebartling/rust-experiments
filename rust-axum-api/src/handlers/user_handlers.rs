use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
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
) -> Result<Json<User>, AppError> {
    // Check if email already exists
    if let Some(_) = User::find_by_email(&pool, &payload.email).await? {
        return Err(AppError::new(
            StatusCode::CONFLICT,
            "Email already exists".to_string(),
        ));
    }

    let user = User::create(&pool, payload).await?;
    Ok(Json(user))
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = User::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    let user = User::update(&pool, id, payload)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let deleted = User::delete(&pool, id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}

pub async fn list_users(
    State(pool): State<PgPool>,
    Query(params): Query<ListQueryParams>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = User::list(&pool, params.limit.into(), params.offset.into()).await?;
    Ok(Json(users))
}
