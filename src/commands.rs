use crate::bot::Bot;
use crate::{database, error};
use serenity::all::*;

impl Bot {
    pub async fn setup_command(ctx: Context) {
        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("register")
                .description("🔐 Register yourself to the bot system across all instances."),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("unregister")
                .description("❌ Unregister yourself from the bot system across all instances."),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("join").description("🤝 Join a specific guild to become a member."),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("leave")
                .description("🚪 Leave the current guild you are a part of."),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("ask")
                .description("❓ Ask a question or initiate a discussion within a guild."),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("add-question")
                .description("➕ Add a new question for the current guild.")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "question",
                        "The question you'd like to add.",
                    )
                    .required(true),
                ),
        )
        .await;

        let _ = Command::create_global_command(
            &ctx.http,
            CreateCommand::new("vote")
                .description("votre for current question")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "username",
                        "Set your answer for current question",
                    )
                    .required(true)
                    .set_autocomplete(true),
                ),
        )
        .await;
    }

    pub async fn handle_command(
        &self,
        ctx: Context,
        command: CommandInteraction,
    ) -> error::Result<()> {
        match command.data.name.as_str() {
            "register" => {
                self.register_user(&ctx, command).await?;
            }
            "unregister" => {
                self.unregister_user(&ctx, command).await?;
            }
            "join" => {
                self.join_guild_user(
                    &ctx,
                    &command,
                    command.guild_id.unwrap(),
                    command.user.clone(),
                )
                .await?;
            }
            "leave" => {
                self.leave_guild_user(
                    &ctx,
                    &command,
                    command.guild_id.unwrap(),
                    command.user.clone(),
                )
                .await?;
            }
            "add-question" => {
                self.add_question(command.data.options.first(), command.guild_id.unwrap())
                    .await?;
                Self::send_success_embed(
                    &ctx,
                    &command,
                    "📝 Question Added!",
                    "Your question has been successfully added.".to_string(),
                    true,
                )
                .await?;
            }
            "ask" => {
                self.add_question(command.data.options.first(), command.guild_id.unwrap())
                    .await?;
                Self::send_success_embed(
                    &ctx,
                    &command,
                    "🧐 Asked a Question!",
                    "A new question has been asked to the guild.".to_string(),
                    true,
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn register_user(&self, ctx: &Context, command: CommandInteraction) -> error::Result<()> {
        if database::User::fetch(&self.pool, command.user.id.into())
            .await?
            .is_none()
        {
            database::User::register(&self.pool, command.user.id.into(), &command.user.name)
                .await?;
            Self::send_success_embed(
                &ctx,
                &command,
                "🎉 User Registered!",
                format!("Successfully registered **{}**!", command.user.name),
                true,
            )
            .await?;
        } else {
            Self::send_error_embed(
                &ctx,
                &command,
                "⚠️ Already Registered",
                &format!("**{}** is already registered!", command.user.name),
                true,
            )
            .await?;
        }
        Ok(())
    }

    async fn unregister_user(
        &self,
        ctx: &Context,
        command: CommandInteraction,
    ) -> error::Result<()> {
        if database::User::fetch(&self.pool, command.user.id.into())
            .await?
            .is_some()
        {
            database::User::delete(&self.pool, command.user.id.into()).await?;
            Self::send_success_embed(
                &ctx,
                &command,
                "🗑️ User Unregistered!",
                format!(
                    "**{}** has been successfully unregistered.",
                    command.user.name
                ),
                true,
            )
            .await?;
        } else {
            Self::send_error_embed(
                &ctx,
                &command,
                "⚠️ Not Registered",
                &format!("**{}** is not registered yet.", command.user.name),
                true,
            )
            .await?;
        }
        Ok(())
    }

    async fn join_guild_user(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        guild_id: GuildId,
        user: User,
    ) -> error::Result<()> {
        if database::User::fetch(&self.pool, user.id.into())
            .await?
            .is_none()
        {
            Self::send_error_embed(
                ctx,
                command,
                "❌ User Not Registered",
                "You need to register first before joining a guild.",
                true,
            )
            .await?;
            return Ok(());
        }
        if database::Guild::fetch(&self.pool, guild_id.into())
            .await?
            .is_none()
        {
            Self::send_error_embed(
                ctx,
                command,
                "❌ Guild Not Registered",
                "This guild is not registered yet.",
                true,
            )
            .await?;
            return Ok(());
        }

        database::GuildUser::register(&self.pool, guild_id.into(), user.id.into()).await?;
        Self::send_success_embed(
            ctx,
            command,
            "✅ Joined Guild!",
            format!("Welcome **{}** to the guild!", user.name),
            false,
        )
        .await?;
        Ok(())
    }

    async fn leave_guild_user(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        guild_id: GuildId,
        user: User,
    ) -> error::Result<()> {
        if database::User::fetch(&self.pool, user.id.into())
            .await?
            .is_none()
        {
            Self::send_error_embed(
                ctx,
                command,
                "❌ User Not Registered",
                "You need to register first before leaving a guild.",
                true,
            )
            .await?;
            return Ok(());
        }
        if database::Guild::fetch(&self.pool, guild_id.into())
            .await?
            .is_none()
        {
            Self::send_error_embed(
                ctx,
                command,
                "❌ Guild Not Registered",
                "This guild is not registered yet.",
                true,
            )
            .await?;
            return Ok(());
        }
        if database::GuildUser::fetch(&self.pool, guild_id.into(), user.id.into())
            .await?
            .is_none()
        {
            Self::send_error_embed(
                ctx,
                command,
                "❌ Not in Guild",
                "You are not a member of this guild. You must join before trying to leave!",
                true,
            )
            .await?;
            return Ok(());
        }

        database::GuildUser::delete(&self.pool, guild_id.into(), user.id.into()).await?;
        Self::send_success_embed(
            ctx,
            command,
            "👋 Left Guild!",
            format!("Goodbye **{}**! We hope to see you again.", user.name),
            false,
        )
        .await?;
        Ok(())
    }

    async fn add_question(
        &self,
        option: Option<&CommandDataOption>,
        guild_id: GuildId,
    ) -> error::Result<()> {
        if let Some(question) = option {
            match &question.value {
                CommandDataOptionValue::String(text) => {
                    let _ = database::Question::register(&self.pool, &text, Some(guild_id.into()))
                        .await?;
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}
