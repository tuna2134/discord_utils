use poise::serenity_prelude::{self as serenity};

use crate::{Data, Error};

pub async fn ready_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!(
                "Logged in as {} ({})",
                data_about_bot.user.name, data_about_bot.user.id
            );
        }
        _ => {}
    }
    Ok(())
}
