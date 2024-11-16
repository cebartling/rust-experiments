use chrono::{DateTime, Utc};
use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CreateSessionData {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub async fn create(pool: &PgPool, data: CreateSessionData) -> Result<Session, sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, 
                expires_at as "expires_at: DateTime<Utc>", 
                created_at as "created_at: DateTime<Utc>"
            "#,
            data.user_id,
            data.token,
            data.expires_at,
        )
            .fetch_one(pool)
            .await
    }

    #[allow(dead_code)]
    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT 
                id,
                user_id,
                token,
                expires_at as "expires_at: DateTime<Utc>",
                created_at as "created_at: DateTime<Utc>"
            FROM sessions
            WHERE token = $1
            "#,
            token
        )
            .fetch_optional(pool)
            .await
    }

    #[allow(dead_code)]
    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT 
                id,
                user_id,
                token,
                expires_at as "expires_at: DateTime<Utc>",
                created_at as "created_at: DateTime<Utc>"
            FROM sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
            .fetch_all(pool)
            .await
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE token = $1
            "#,
            token
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn delete_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < CURRENT_TIMESTAMP
            "#
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    #[allow(dead_code)]
    pub async fn delete_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE user_id = $1
            "#,
            user_id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    #[allow(dead_code)]
    pub async fn is_valid(&self, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM sessions
                WHERE token = $1
                AND expires_at > CURRENT_TIMESTAMP
            )
            "#,
            self.token
        )
            .fetch_one(pool)
            .await?;

        Ok(result.unwrap_or(false))
    }

    #[allow(dead_code)]
    pub async fn extend_expiry(
        pool: &PgPool,
        token: &str,
        new_expiry: DateTime<Utc>,
    ) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
            UPDATE sessions
            SET expires_at = $1
            WHERE token = $2
            AND expires_at > CURRENT_TIMESTAMP
            RETURNING id, user_id, token, expires_at as "expires_at: DateTime<Utc>", created_at as "created_at: DateTime<Utc>"
            "#,
            new_expiry,
            token
        )
            .fetch_optional(pool)
            .await
    }

    #[allow(dead_code)]
    pub async fn clean_expired_sessions(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < CURRENT_TIMESTAMP
            "#
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}

// Helper functions for token generation and session management
impl Session {
    #[allow(dead_code)]
    pub fn generate_token() -> String {
        let random_bytes: Vec<u8> = (0..32).map(|_| random()).collect();
        hex::encode(random_bytes)
    }

    #[allow(dead_code)]
    pub fn default_expiry() -> DateTime<Utc> {
        Utc::now() + chrono::Duration::hours(24)
    }

    #[allow(dead_code)]
    pub async fn create_for_user(
        pool: &PgPool,
        user_id: Uuid,
        expiry: Option<DateTime<Utc>>,
    ) -> Result<Session, sqlx::Error> {
        let session_data = CreateSessionData {
            user_id,
            token: Self::generate_token(),
            expires_at: expiry.unwrap_or_else(Self::default_expiry),
        };

        Session::create(pool, session_data).await
    }
}
