use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

// --- 問題データ(JSON)用の定義 ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionMode {
    Selection, // 選択
    TextInput, // 完全自由入力
    MathSimple, // ソフトキーボード,簡単な数式
    MathAdvanced, // ソフトキーボード,関数など高度な数式
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub mode: QuestionMode,
    pub question: String,

    pub options: Option<Vec<String>>, // 選択肢(選択式のみ)
    pub answer: String,                // 正答
    pub display_answer: Option<String>,   // 解説表示用正解(LaTeXなど)
    pub explanation: Option<String>,     // 解説

    #[serde(default)] // JSONにキーがなければfalseになる
    pub auto_grade:bool,              // 自動採点フラグ
}

// --- 履歴データベース用データ定義 ---

#[derive(Debug, Serialize, FromRow)]
pub struct HistoryRecord {
    pub id: i64,
    pub question_id: String,
    pub mode: String,
    pub is_correct: bool,
    pub user_input: Option<String>,
    pub answered_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateHistoryRequest {
    pub question_id: String,
    pub mode: QuestionMode,
    pub is_correct: bool,
    pub user_input: Option<String>,
}