use axum::{Json, extract::State};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

mod middleware;
pub use middleware::api_key_middleware;

use crate::database::Question;

#[derive(Debug, Deserialize)]
pub struct CreateQuestionRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateQuestionResponse {
    pub question_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

pub async fn create_global_question(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuestionRequest>,
) -> Result<Json<CreateQuestionResponse>, (axum::http::StatusCode, String)> {
    let question = Question::register(&pool, &payload.content, None)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CreateQuestionResponse {
        question_id: question.question_id,
        content: question.content,
        created_at: question.created_at,
    }))
}
