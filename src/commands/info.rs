use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = {
        let shard_manager = ctx.framework().shard_manager();

        let runners = shard_manager.runners.lock().await;
        let shard = runners.get(&ctx.serenity_context().shard_id);

        if let Some(duration) = shard.unwrap().latency {
            format!("{:.2}", duration.as_millis())
        } else {
            "???".to_string()
        }
    };
    ctx.reply(format!("Pong! 🏓 latency {}ms", latency)).await?;
    Ok(())
}
