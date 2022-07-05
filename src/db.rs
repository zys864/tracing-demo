use sqlx::mysql::MySqlPoolOptions;
use tracing::info;

pub type DbPool = sqlx::MySqlPool;

pub async fn db_pool() -> sqlx::Result<DbPool> {
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;
    info!("Successful connect to sql");
    Ok(pool)
}
