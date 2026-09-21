// Build logging functionality

use crate::db::connection::establish_connection;
use crate::db::schema::create_tables;

#[cfg(feature = "postgres")]
#[derive(Clone)]
pub struct BuildLogger {
    pub pool: sqlx::PgPool,
}

#[cfg(feature = "sqlite")]
#[derive(Clone)]
pub struct BuildLogger {
    pub pool: sqlx::SqlitePool,
}

impl BuildLogger {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = establish_connection(database_url).await?;
        create_tables(&pool).await?;
        Ok(BuildLogger { pool })
    }

    pub async fn log_build(
        &self,
        project_path: &str,
        language: &str,
        artifact_path: &str,
        size: u64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO builds (project_path, language, artifact_path, size_bytes) VALUES ($1, $2, $3, $4)",
        )
        .bind(project_path)
        .bind(language)
        .bind(artifact_path)
        .bind(size as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
