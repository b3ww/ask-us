use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Guild {
    pub discord_guild_id: i64,
    pub channel_id: Option<i64>,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Guild {
    pub async fn fetch(pool: &PgPool, discord_guild_id: i64) -> Result<Option<Self>, Error> {
        let guild = sqlx::query_as!(
            Guild,
            "SELECT discord_guild_id, channel_id, name, created_at FROM guilds WHERE discord_guild_id = $1",
            discord_guild_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(guild)
    }

    pub async fn register(
        pool: &PgPool,
        discord_guild_id: i64,
        channel_id: i64,
        name: Option<&str>,
    ) -> Result<Self, Error> {
        let guild = sqlx::query_as!(
            Guild,
            "INSERT INTO guilds (discord_guild_id, channel_id, name) VALUES ($1, $2, $3) RETURNING discord_guild_id, channel_id, name, created_at",
            discord_guild_id,
            channel_id,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(guild)
    }

    pub async fn update_name(
        pool: &PgPool,
        discord_guild_id: i64,
        new_name: Option<&str>,
    ) -> Result<Self, Error> {
        let guild = sqlx::query_as!(
            Guild,
            "UPDATE guilds SET name = $1 WHERE discord_guild_id = $2 RETURNING discord_guild_id, channel_id, name, created_at",
            new_name,
            discord_guild_id
        )
        .fetch_one(pool)
        .await?;

        Ok(guild)
    }

    pub async fn update_channel_id(
        pool: &PgPool,
        discord_guild_id: i64,
        channel_id: i64,
    ) -> Result<Self, Error> {
        let guild = sqlx::query_as!(
            Guild,
            "UPDATE guilds SET channel_id = $1 WHERE discord_guild_id = $2 RETURNING discord_guild_id, channel_id, name, created_at",
            channel_id,
            discord_guild_id
        )
        .fetch_one(pool)
        .await?;

        Ok(guild)
    }

    pub async fn delete(pool: &PgPool, discord_guild_id: i64) -> Result<u64, Error> {
        let deleted_count = sqlx::query!(
            "DELETE FROM guilds WHERE discord_guild_id = $1",
            discord_guild_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(deleted_count) // TODO update GuildUser
    }
}
