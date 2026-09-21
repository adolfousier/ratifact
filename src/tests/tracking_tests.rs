// Tracking tests

use crate::db::connection::establish_connection;
use crate::tracking::logger::BuildLogger;

#[cfg(feature = "postgres")]
#[tokio::test]
async fn test_log_build() {
    let database_url = "postgres://user:password@localhost:25851/ratifact";
    if let Ok(logger) = BuildLogger::new(database_url).await {
        logger
            .log_build("/path/to/project", "rust", "/path/to/target", 1024)
            .await
            .unwrap();
        // Check if inserted
        let pool = establish_connection(database_url).await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM builds")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(count.0 >= 1);
    }
}

/// The default (sqlite) build must exercise the real insert path — this is
/// where the `SERIAL PRIMARY KEY` schema bug hid, since the postgres test
/// above is skipped when no local database is reachable.
#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_log_build() {
    let dir = tempfile::tempdir().unwrap();
    let database_url = format!(
        "sqlite://{}/ratifact_test.db?mode=rwc",
        dir.path().display()
    );
    let logger = BuildLogger::new(&database_url).await.unwrap();
    logger
        .log_build("/path/to/project", "rust", "/path/to/target", 1024)
        .await
        .unwrap();
    // Check if inserted
    let pool = establish_connection(&database_url).await.unwrap();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM builds")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1);
}
