use sqlx::postgres::PgPool;
use std::error::Error;

pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    Ok(())
}