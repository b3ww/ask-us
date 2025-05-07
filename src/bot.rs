use serenity::all::*;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::error;

async fn get_db_pool() -> Result<PgPool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini dans le fichier .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub struct Bot {
    pub pool: PgPool,
}

impl Bot {
    pub async fn try_new() -> anyhow::Result<Self> {
        Ok(Bot {
            pool: get_db_pool().await?,
        })
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        Self::setup_command(ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let _ = self.hanlde_interactions(ctx, interaction).await;
        ()
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        match self.guild_setup_channel(ctx, guild, is_new).await {
            Err(e) => error!("{}", e.to_string()),
            _ => {}
        }
    }

    // async fn guild_delete(&self, ctx: Context, _i: UnavailableGuild, guild: Option<Guild>) {
    // Some(guild)
    // }
}
