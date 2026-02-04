use sqlx::sqlite::{SqlitePoolOptions, Sqlite, SqliteConnectOptions};
use sqlx::{Pool, ConnectOptions};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub type DbResult<T> = Result<T, sqlx::Error>;

pub async fn initialize_database(app_data_dir: &PathBuf) -> DbResult<Pool<Sqlite>> {
    if !app_data_dir.exists() {
        fs::create_dir_all(app_data_dir).expect("データディレクトリの作成に失敗しました");
    }

    // DBファイルのパス作成
    let db_path = app_data_dir.join("gori_memory.db");

    let connect_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connect_options)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            question_id TEXT NOT NULL,
            mode TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            user_input TEXT,
            answered_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// 履歴追加関数
use crate::models::{CreateHistoryRequest, QuestionMode};

pub async fn add_history(pool: &Pool<Sqlite>, data: CreateHistoryRequest) -> DbResult<i64> {
    let mode_str = serde_json::to_string(&data.mode).unwrap_or_default().replace("\"", "");
    
    let result = sqlx::query("INSERT INTO history (question_id, mode, is_correct, user_input) VALUES ($1, $2, $3, $4)",
    )
    .bind(data.question_id)
    .bind(mode_str)
    .bind(data.is_correct)
    .bind(data.user_input)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}