use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub discord_user_id: i64,
    pub username: String,
}

impl User {
    pub async fn fetch(pool: &PgPool, discord_user_id: i64) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT discord_user_id, username FROM users WHERE discord_user_id = $1",
        )
        .bind(discord_user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn register(
        pool: &PgPool,
        discord_user_id: i64,
        username: &str,
    ) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (discord_user_id, username) VALUES ($1, $2) RETURNING discord_user_id, username",
        )
        .bind(discord_user_id)
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_username(
        pool: &PgPool,
        discord_user_id: i64,
        new_username: &str,
    ) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET username = $1 WHERE discord_user_id = $2 RETURNING discord_user_id, username",
        )
        .bind(new_username)
        .bind(discord_user_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, discord_user_id: i64) -> Result<u64, Error> {
        let deleted_count = sqlx::query("DELETE FROM users WHERE discord_user_id = $1")
            .bind(discord_user_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted_count) // TODO update GuildUser
    }
}
