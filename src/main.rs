mod api;
mod bot;
mod channel;
mod commands;
mod database;
mod error;
mod interactions;
mod messages;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    #[cfg(feature = "bot")]
    run_bot().await?;

    #[cfg(feature = "api")]
    run_api().await?;

    #[cfg(all(feature = "bot", feature = "api"))]
    tokio::try_join!(run_bot(), run_api())?;

    Ok(())
}

#[cfg(feature = "bot")]
async fn run_bot() -> anyhow::Result<()> {
    use serenity::model::gateway::GatewayIntents;
    use std::env;

    let bot = bot::Bot::try_new().await?;
    let mut client = serenity::Client::builder(
        &env::var("DISCORD_TOKEN")?,
        GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MEMBERS,
    )
    .event_handler(bot)
    .await?;

    tokio::spawn(async move {
        if let Err(e) = client.start().await {
            eprintln!("Bot client error: {:?}", e);
        }
    });

    Ok(())
}

#[cfg(feature = "api")]
async fn run_api() -> anyhow::Result<()> {
    use api::{api_key_middleware, create_global_question};
    use axum::{Router, middleware, routing::post};
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    let pool = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let app = Router::new()
        .route("/question", post(create_global_question))
        .layer(middleware::from_fn(api_key_middleware))
        .with_state(pool);

    let addr = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
