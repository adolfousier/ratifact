// Database connection management
#[cfg(feature = "postgres")]
pub async fn establish_connection(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    sqlx::PgPool::connect(database_url).await
}

#[cfg(feature = "sqlite")]
pub async fn establish_connection(database_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    sqlx::SqlitePool::connect(database_url).await
}
