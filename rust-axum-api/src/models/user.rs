use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, 
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, data: CreateUserRequest) -> Result<User, sqlx::Error> {
        let password_hash = bcrypt::hash(data.password.as_bytes(), 10)
            .expect("Failed to hash password");

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, full_name)
            VALUES ($1, $2, $3)
            RETURNING id, email, password_hash, full_name, created_at, updated_at
            "#,
            data.email,
            password_hash,
            data.full_name
        )
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        data: UpdateUserRequest,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = User::find_by_id(pool, id).await?;

        if let Some(user) = user {
            let password_hash = if let Some(password) = data.password {
                bcrypt::hash(password.as_bytes(), 10)
                    .expect("Failed to hash password")
            } else {
                user.password_hash
            };

            let email = data.email.unwrap_or(user.email);
            let full_name = data.full_name.unwrap_or(user.full_name);

            sqlx::query_as!(
                User,
                r#"
                UPDATE users
                SET email = $1, password_hash = $2, full_name = $3
                WHERE id = $4
                RETURNING id, email, password_hash, full_name, created_at, updated_at
                "#,
                email,
                password_hash,
                full_name,
                id
            )
                .fetch_optional(pool)
                .await
        } else {
            Ok(None)
        }
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
            .fetch_all(pool)
            .await
    }
}
