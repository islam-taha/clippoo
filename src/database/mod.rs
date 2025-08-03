use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub is_default: bool,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;

        // Ensure directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Touch the file to ensure it exists
        if !db_path.exists() {
            std::fs::File::create(&db_path)?;
        }

        let db_url = format!("sqlite:{}", db_path.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
        .await?;

        let db = Self { pool };
        db.init_schema().await?;
        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?;
        Ok(data_dir.join("clippoo").join("clipboard.db"))
    }

    async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL UNIQUE,
                timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                is_default BOOLEAN NOT NULL DEFAULT FALSE
            );

            CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_history(timestamp DESC);
            CREATE INDEX IF NOT EXISTS idx_default ON clipboard_history(is_default);
            "#,
        )
            .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_entry(&self, content: String) -> Result<()> {
        // First, check if this content already exists
        let existing = sqlx::query_as::<_, (i64,)>(
            "SELECT id FROM clipboard_history WHERE content = ?1"
        )
            .bind(&content)
            .fetch_optional(&self.pool)
        .await?;

        if let Some((id,)) = existing {
            // Update timestamp for existing entry
            sqlx::query(
                "UPDATE clipboard_history SET timestamp = CURRENT_TIMESTAMP WHERE id = ?1"
            )
                .bind(id)
                .execute(&self.pool)
            .await?;
        } else {
            // Insert new entry and set as default
            let mut tx = self.pool.begin().await?;

            // Clear current default
            sqlx::query("UPDATE clipboard_history SET is_default = FALSE WHERE is_default = TRUE")
                .execute(&mut *tx)
            .await?;

            // Insert new entry as default
            sqlx::query(
                "INSERT INTO clipboard_history (content, is_default) VALUES (?1, TRUE)"
            )
                .bind(&content)
                .execute(&mut *tx)
            .await?;

            tx.commit().await?;

            // Clean up old entries (keep only last 50)
            self.cleanup_old_entries().await?;
        }

        Ok(())
    }

    async fn cleanup_old_entries(&self) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM clipboard_history
            WHERE id NOT IN (
                SELECT id FROM clipboard_history
                ORDER BY timestamp DESC
                LIMIT 50
            )
            "#
        )
            .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_entries(&self, limit: i64) -> Result<Vec<ClipboardEntry>> {
        let entries = sqlx::query_as::<_, ClipboardEntry>(
            "SELECT id, content, timestamp, is_default FROM clipboard_history
ORDER BY timestamp DESC
LIMIT ?1"
        )
            .bind(limit)
            .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    pub async fn get_default_entry(&self) -> Result<Option<ClipboardEntry>> {
        let entry = sqlx::query_as::<_, ClipboardEntry>(
            "SELECT id, content, timestamp, is_default FROM clipboard_history
WHERE is_default = TRUE
LIMIT 1"
        )
            .fetch_optional(&self.pool)
        .await?;

        Ok(entry)
    }

    pub async fn set_default_entry(&self, id: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Clear current default
        sqlx::query("UPDATE clipboard_history SET is_default = FALSE WHERE is_default = TRUE")
            .execute(&mut *tx)
        .await?;

        // Set new default
        sqlx::query("UPDATE clipboard_history SET is_default = TRUE WHERE id = ?1")
            .bind(id)
            .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
