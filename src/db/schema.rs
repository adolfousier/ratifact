// Database schema definitions

#[cfg(feature = "postgres")]
use sqlx::PgPool;

#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

#[cfg(feature = "postgres")]
pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS builds (
            id SERIAL PRIMARY KEY,
            project_path TEXT NOT NULL,
            language TEXT NOT NULL,
            build_time TIMESTAMPTZ DEFAULT NOW(),
            artifact_path TEXT NOT NULL,
            size_bytes BIGINT
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(feature = "postgres")]
pub async fn get_old_artifact_paths(
    pool: &PgPool,
    retention_days: u32,
) -> Result<Vec<String>, sqlx::Error> {
    let artifacts = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT artifact_path FROM builds WHERE build_time < NOW() - INTERVAL '1 day' * $1"
    )
    .bind(retention_days as i32)
    .fetch_all(pool)
    .await?;

    Ok(artifacts.into_iter().map(|(path,)| path).collect())
}

#[cfg(feature = "postgres")]
pub async fn delete_old_builds_from_db(
    pool: &PgPool,
    retention_days: u32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM builds WHERE build_time < NOW() - INTERVAL '1 day' * $1")
        .bind(retention_days as i32)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

#[cfg(feature = "sqlite")]
pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS builds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_path TEXT NOT NULL,
            language TEXT NOT NULL,
            build_time TEXT DEFAULT CURRENT_TIMESTAMP,
            artifact_path TEXT NOT NULL,
            size_bytes INTEGER
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(feature = "sqlite")]
pub async fn get_old_artifact_paths(
    pool: &SqlitePool,
    retention_days: u32,
) -> Result<Vec<String>, sqlx::Error> {
    // build_time is stored as UTC TEXT ('YYYY-MM-DD HH:MM:SS'), so the
    // retention cutoff must be computed with SQLite's datetime() as well.
    let artifacts = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT artifact_path FROM builds WHERE build_time < datetime('now', '-' || $1 || ' days')"
    )
    .bind(retention_days as i32)
    .fetch_all(pool)
    .await?;

    Ok(artifacts.into_iter().map(|(path,)| path).collect())
}

#[cfg(feature = "sqlite")]
pub async fn delete_old_builds_from_db(
    pool: &SqlitePool,
    retention_days: u32,
) -> Result<u64, sqlx::Error> {
    let result =
        sqlx::query("DELETE FROM builds WHERE build_time < datetime('now', '-' || $1 || ' days')")
            .bind(retention_days as i32)
            .execute(pool)
            .await?;

    Ok(result.rows_affected())
}
