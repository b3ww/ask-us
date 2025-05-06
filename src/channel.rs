use crate::bot::Bot;
use crate::database;
use crate::error;
use serenity::all::*;
use serenity::client::Context;

async fn get_or_create_channel(
    ctx: &Context,
    guild: &Guild,
    name: &str,
) -> error::Result<ChannelId> {
    let channels = guild.channels(&ctx.http).await?;
    if let Some(channel) = channels.into_iter().find(|(_, c)| c.name == name) {
        Ok(channel.0)
    } else {
        Ok(guild
            .create_channel(&ctx, CreateChannel::new("ask-us").kind(ChannelType::Text))
            .await?
            .id)
    }
}

impl Bot {
    pub async fn guild_setup_channel(
        &self,
        ctx: Context,
        guild: Guild,
        _is_new: Option<bool>,
    ) -> error::Result<()> {
        if let Some(guild_db) = database::Guild::fetch(&self.pool, guild.id.into()).await? {
            if guild_db.channel_id.is_none() {
                let _ = database::Guild::update_channel_id(
                    &self.pool,
                    guild.id.into(),
                    get_or_create_channel(&ctx, &guild, "ask-us").await?.into(),
                )
                .await;
            }
            if guild_db.name.is_none() {
                let _ =
                    database::Guild::update_name(&self.pool, guild.id.into(), Some(&guild.name))
                        .await;
            }
        } else {
            let _ = database::Guild::register(
                &self.pool,
                guild.id.into(),
                get_or_create_channel(&ctx, &guild, "ask-us").await?.into(),
                Some(&guild.name),
            )
            .await;
        }
        Ok(())
    }
}
