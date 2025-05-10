mod commands;
mod handlers;
use handlers::error;
use poise::serenity_prelude::{self as serenity};
use tokio::try_join;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::info::ping(), commands::clone::clone_category()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(async move {
                    try_join!(handlers::ready::ready_handler(ctx, event, framework, data),)?;
                    Ok(())
                })
            },
            initialize_owners: true,
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = error::on_error(error).await {
                        tracing::error!("Error while handling error: {}", e);
                    }
                })
            },
            prefix_options: poise::PrefixFrameworkOptions {
                mention_as_prefix: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
