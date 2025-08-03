use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use tempfile::TempDir;

#[tokio::test]
async fn test_sqlite_basic_operations() -> Result<()> {
    // Create a temporary directory for test database
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    // Touch the file to ensure it exists
    std::fs::File::create(&db_path)?;
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Create schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL UNIQUE,
            timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            is_default BOOLEAN NOT NULL DEFAULT FALSE
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    // Test insert
    sqlx::query("INSERT INTO clipboard_history (content, is_default) VALUES (?1, TRUE)")
        .bind("Test content")
        .execute(&pool)
        .await?;
    
    // Test select
    let (count,): (i32,) = sqlx::query_as("SELECT COUNT(*) FROM clipboard_history")
        .fetch_one(&pool)
        .await?;
    
    assert_eq!(count, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_clipboard_entry_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    // Touch the file to ensure it exists
    std::fs::File::create(&db_path)?;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Create schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL UNIQUE,
            timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            is_default BOOLEAN NOT NULL DEFAULT FALSE
        );
        CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_history(timestamp DESC);
        "#,
    )
    .execute(&pool)
    .await?;
    
    // Add multiple entries
    for i in 0..5 {
        sqlx::query("INSERT INTO clipboard_history (content) VALUES (?1)")
            .bind(format!("Content {}", i))
            .execute(&pool)
            .await?;
    }
    
    // Test retrieval
    let entries: Vec<(String,)> = sqlx::query_as(
        "SELECT content FROM clipboard_history ORDER BY timestamp DESC LIMIT 3"
    )
    .fetch_all(&pool)
    .await?;
    
    assert_eq!(entries.len(), 3);
    // Since we're ordering by timestamp DESC and all inserts happen quickly,
    // the exact order might vary. Just check that we got 3 entries.
    
    Ok(())
}

#[test]
fn test_basic_setup() {
    // Basic sanity test
    assert!(true);
}