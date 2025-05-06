use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub question_id: i32,
    pub content: String,
    pub discord_guild_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

impl Question {
    pub async fn fetch_by_id(pool: &PgPool, question_id: i32) -> Result<Option<Self>, Error> {
        let question = sqlx::query_as!(
            Question,
            "SELECT question_id, content, discord_guild_id, created_at FROM questions WHERE question_id = $1",
            question_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(question)
    }

    pub async fn register(
        pool: &PgPool,
        content: &str,
        discord_guild_id: Option<i64>,
    ) -> Result<Self, Error> {
        let question = sqlx::query_as!(
            Question,
            "INSERT INTO questions (content, discord_guild_id) VALUES ($1, $2) RETURNING question_id, content, discord_guild_id, created_at",
            content,
            discord_guild_id
        )
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    pub async fn fetch_unanswered_for_guild(
        pool: &PgPool,
        guild_id: i64,
    ) -> Result<Vec<Self>, Error> {
        let questions = sqlx::query_as!(
            Question,
            r#"
            SELECT q.question_id, q.content, q.discord_guild_id, q.created_at
            FROM questions q
            LEFT JOIN answers a ON q.question_id = a.question_id
            WHERE q.discord_guild_id = $1 AND a.answer_id IS NULL
            "#,
            guild_id
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }
}
