// DB tests

use crate::db::connection::establish_connection;
use crate::db::schema::{create_tables, delete_old_builds_from_db, get_old_artifact_paths};

#[cfg(feature = "postgres")]
use sqlx::PgPool;

#[cfg(feature = "postgres")]
#[tokio::test]
async fn test_establish_connection() {
    let database_url = "postgres://user:password@localhost:25851/ratifact";
    if let Ok(pool) = establish_connection(database_url).await {
        assert!(!pool.is_closed());
    }
}

#[cfg(feature = "postgres")]
#[tokio::test]
async fn test_create_tables() {
    let database_url = "postgres://user:password@localhost:25851/ratifact";
    // For test, assume DB is running, or use testcontainers, but for now skip if not connected
    if let Ok(pool) = PgPool::connect(database_url).await {
        create_tables(&pool).await.unwrap();
        // Check if table exists
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'builds'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(row.0 > 0);
    }
}

#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

/// Fresh temp-file database, schema applied. No repo-cwd pollution.
#[cfg(feature = "sqlite")]
async fn setup_sqlite() -> (tempfile::TempDir, SqlitePool) {
    let dir = tempfile::tempdir().unwrap();
    let url = format!(
        "sqlite://{}/ratifact_test.db?mode=rwc",
        dir.path().display()
    );
    let pool = SqlitePool::connect(&url).await.unwrap();
    (dir, pool)
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_establish_connection() {
    let (_dir, _pool) = setup_sqlite().await;
    // establish_connection must reach a fresh sqlite file without error.
    let url = format!(
        "sqlite://{}/ratifact_test.db?mode=rwc",
        _dir.path().display()
    );
    let pool = establish_connection(&url).await.unwrap();
    assert!(!pool.is_closed());
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_create_tables() {
    let (_dir, pool) = setup_sqlite().await;
    create_tables(&pool).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE name = 'builds'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(row.0 > 0);
}

/// Regression: inserts must omit `id` (the app never supplies one) and the
/// retention cutoff must use SQLite datetime math. Guards against the
/// `SERIAL PRIMARY KEY` schema and the copied-over `NOW() - INTERVAL` SQL.
#[cfg(feature = "sqlite")]
async fn insert_build(pool: &SqlitePool, artifact_path: &str, age_days: i32) {
    sqlx::query(
        "INSERT INTO builds (project_path, language, build_time, artifact_path, size_bytes)
         VALUES ($1, $2, datetime('now', $3), $4, $5)",
    )
    .bind("/srv/project")
    .bind("rust")
    .bind(format!("-{} days", age_days))
    .bind(artifact_path)
    .bind(1024i64)
    .execute(pool)
    .await
    .unwrap();
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_retention_queries() {
    let (_dir, pool) = setup_sqlite().await;
    create_tables(&pool).await.unwrap();

    insert_build(&pool, "/srv/project/target/release/app", 100).await; // older than retention
    insert_build(&pool, "/srv/project/target/debug/app", 1).await; // fresher than retention

    let old = get_old_artifact_paths(&pool, 30).await.unwrap();
    assert_eq!(old, vec!["/srv/project/target/release/app".to_string()]);

    let deleted = delete_old_builds_from_db(&pool, 30).await.unwrap();
    assert_eq!(deleted, 1);

    let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM builds")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(remaining.0, 1);
}
