use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::database::Guild;
use crate::database::User;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildUser {
    pub discord_guild_id: i64,
    pub discord_user_id: i64,
}

impl GuildUser {
    pub async fn fetch(
        pool: &PgPool,
        discord_guild_id: i64,
        discord_user_id: i64,
    ) -> Result<Option<GuildUser>, Error> {
        sqlx::query_as::<_, GuildUser>(
            "SELECT g.discord_guild_id, g.discord_user_id
                  FROM guild_users g
                  WHERE g.discord_guild_id = $1 AND g.discord_user_id = $2",
        )
        .bind(discord_guild_id)
        .bind(discord_user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn register(
        pool: &PgPool,
        discord_guild_id: i64,
        discord_user_id: i64,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO guild_users (discord_guild_id, discord_user_id) VALUES ($1, $2)")
            .bind(discord_guild_id)
            .bind(discord_user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(
        pool: &PgPool,
        discord_guild_id: i64,
        discord_user_id: i64,
    ) -> Result<u64, Error> {
        let deleted_count = sqlx::query(
            "DELETE FROM guild_users WHERE discord_guild_id = $1 AND discord_user_id = $2",
        )
        .bind(discord_guild_id)
        .bind(discord_user_id)
        .execute(pool)
        .await?
        .rows_affected();

        Ok(deleted_count)
    }

    #[allow(dead_code)]
    pub async fn get_guilds_for_user(
        pool: &PgPool,
        discord_user_id: i64,
    ) -> Result<Vec<Guild>, Error> {
        let guilds = sqlx::query_as::<_, Guild>(
            "SELECT g.discord_guild_id, g.channel_id, g.name, g.created_at
            FROM guild_users gu
            JOIN guilds g ON gu.discord_guild_id = g.discord_guild_id
            WHERE gu.discord_user_id = $1",
        )
        .bind(discord_user_id)
        .fetch_all(pool)
        .await?;

        Ok(guilds)
    }

    pub async fn get_users_for_guild(
        pool: &PgPool,
        discord_guild_id: i64,
    ) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT u.discord_user_id, u.username
            FROM guild_users gu
            JOIN users u ON gu.discord_user_id = u.discord_user_id
            WHERE gu.discord_guild_id = $1",
        )
        .bind(discord_guild_id)
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
