use crate::{Context, Error};
use poise::{
    serenity_prelude as serenity, serenity_prelude::{ChannelType, CreateChannel},
    CreateReply,
};

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn clone_category(
    ctx: Context<'_>,
    #[description = "複製元のカテゴリ"]
    #[channel_types("Category")]
    category: serenity::GuildChannel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let response = ctx.reply("カテゴリを複製中...").await?;

    let builder = CreateChannel::new(&category.name)
        .kind(ChannelType::Category)
        .permissions(category.permission_overwrites.clone())
        .position(category.position);

    let guild = ctx.guild().unwrap().clone();
    guild
        .create_channel(&ctx.serenity_context().http, builder)
        .await?;

    let message = CreateReply::default().content(format!(
        "✅ カテゴリ「`{}`」の複製が完了しました！",
        category.name
    ));
    response.edit(ctx, message).await?;
    Ok(())
}
