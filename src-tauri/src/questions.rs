use std::fs;
use crate::models::Question;

type Result<T> = std::result::Result<T, String>;

pub fn load_questions_from_file(file_path: &str) -> Result<Vec<Question>> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;
    let questions: Vec<Question> = serde_json::from_str(&content)
        .map_err(|e| format!("JSONの解析に失敗しました: {}", e))?;

    Ok(questions)
}