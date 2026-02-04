// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod models;
pub mod database;
pub mod questions;

use tauri::{Manager, State};
use sqlx::{Pool, Sqlite};
use models::{CreateHistoryRequest, Question};

#[tauri::command]
async fn save_history_command(
    pool: State<'_, Pool<Sqlite>>,
    request: CreateHistoryRequest
) -> Result<String, String> {
    let id = database::add_history(&*pool, request)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Record saved with ID: {}", id))
}

#[tauri::command]
async fn load_questions_command(file_path: String) -> Result<Vec<Question>, String> {
    questions::load_questions_from_file(&file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let app_data_dir = app.path().app_data_dir().unwrap();
            println!("Database Path: {:?}", app_data_dir);
            let pool = tauri::async_runtime::block_on(async {
                database::initialize_database(&app_data_dir).await
            }).expect("failed to initialize database");
            app.manage(pool);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_history_command,
            load_questions_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}