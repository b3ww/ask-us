use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Answer {
    pub answer_id: i32,
    pub question_id: i32,
    pub answered_by: i64,
    pub answer_user_id: i64,
    pub created_at: NaiveDateTime,
}

impl Answer {
    pub async fn register(
        pool: &PgPool,
        question_id: i32,
        answered_by: i64,
        answer_user_id: i64,
    ) -> Result<Self, Error> {
        let answer = sqlx::query_as::<_, Answer>(
            "INSERT INTO answers (question_id, answered_by, answer_user_id) 
            VALUES ($1, $2, $3) RETURNING answer_id, question_id, answered_by, answer_user_id, created_at")
            .bind(question_id)
            .bind(answered_by)
            .bind(answer_user_id)
        .fetch_one(pool)
        .await?;

        Ok(answer)
    }

    pub async fn fetch_by_question(pool: &PgPool, question_id: i32) -> Result<Vec<Self>, Error> {
        let answers = sqlx::query_as::<_, Answer>(
            "SELECT answer_id, question_id, answered_by, answer_user_id, created_at FROM answers WHERE question_id = $1"
        )
        .bind(question_id)
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }

    pub async fn fetch_by_question_and_guild(
        pool: &PgPool,
        question_id: i32,
        discord_guild_id: i64,
    ) -> Result<Vec<Self>, Error> {
        let answers = sqlx::query_as::<_, Answer>(
            r#"
            SELECT a.answer_id, a.question_id, a.answered_by, a.answer_user_id, a.created_at
            FROM answers a
            JOIN questions q ON a.question_id = q.question_id
            WHERE a.question_id = $1 AND q.discord_guild_id = $2
            "#,
        )
        .bind(question_id)
        .bind(discord_guild_id)
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }
}
