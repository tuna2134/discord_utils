use poise::{serenity_prelude as serenity, CreateReply};

pub async fn on_error<U, E: std::fmt::Display + std::fmt::Debug>(
    error: poise::FrameworkError<'_, U, E>,
) -> Result<(), serenity::Error> {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            eprintln!("Error in user data setup: {}", error);
        }
        poise::FrameworkError::EventHandler { error, event, .. } => tracing::error!(
            "User event event handler encountered an error on {} event: {}",
            event.snake_case_name(),
            error
        ),
        poise::FrameworkError::Command { ctx, error, .. } => {
            let error = error.to_string();
            eprintln!("An error occured in a command: {}", error);
            ctx.say("コマンドの実行中にエラーが発生しました。").await?;
        }
        poise::FrameworkError::SubcommandRequired { ctx } => {
            let subcommands = ctx
                .command()
                .subcommands
                .iter()
                .map(|s| &*s.name)
                .collect::<Vec<_>>();
            let response = format!(
                "サブコマンドを指定してください。コマンド: {}",
                subcommands.join(", ")
            );
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::CommandPanic {
            ctx, payload: _, ..
        } => {
            let response = "コマンドの実行中にエラーが発生しました。";
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::ArgumentParse { ctx, input, .. } => {
            let response = if let Some(input) = input {
                format!(
                    "コマンド引数の解析に失敗しました。 コマンド: `{}` 引数: `{}`",
                    ctx.command().name,
                    input,
                )
            } else {
                format!(
                    "コマンド引数の解析に失敗しました。 コマンド: `{}`",
                    ctx.command().name,
                )
            };
            ctx.reply(response).await?;
        }
        poise::FrameworkError::CommandStructureMismatch {
            ctx, description, ..
        } => {
            tracing::error!(
                "Error: failed to deserialize interaction arguments for `/{}`: {}",
                ctx.command.name,
                description,
            );
        }
        poise::FrameworkError::CommandCheckFailed { ctx, error, .. } => {
            tracing::error!(
                "A command check failed in command {} for user {}: {:?}",
                ctx.command().name,
                ctx.author().name,
                error,
            );
        }
        poise::FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            let response = format!(
                "レート制限中です。 {} 秒後に再実行してください。",
                remaining_cooldown.as_secs()
            );
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let response = format!(
                "コマンドを実行できませんでした。Botの権限を確認してください。: {}",
                missing_permissions,
            );
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let response = if let Some(missing_permissions) = missing_permissions {
                format!(
                    "コマンドを実行する権限がありません。 コマンド: `{}` 必要な権限: {}",
                    ctx.command().name,
                    missing_permissions,
                )
            } else {
                format!(
                    "たぶん？コマンドを実行する権限がありません。 コマンド: `{}`",
                    ctx.command().name,
                )
            };
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::NotAnOwner { ctx, .. } => {
            let response = "コマンドを実行する権限がありません。";
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::GuildOnly { ctx, .. } => {
            let response = "サーバーでのみ実行可能なコマンドです。";
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::DmOnly { ctx, .. } => {
            let response = "DMでのみ実行可能なコマンドです。";
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::NsfwOnly { ctx, .. } => {
            let response = "NSFWチャンネルでのみ実行可能なコマンドです。";
            ctx.send(CreateReply::default().content(response).ephemeral(true))
                .await?;
        }
        poise::FrameworkError::DynamicPrefix { error, msg, .. } => {
            tracing::error!(
                "Dynamic prefix failed for message {:?}: {}",
                msg.content,
                error
            );
        }
        poise::FrameworkError::UnknownCommand {
            msg_content,
            prefix,
            ..
        } => {
            tracing::warn!(
                "Recognized prefix `{}`, but didn't recognize command name in `{}`",
                prefix,
                msg_content,
            );
        }
        poise::FrameworkError::UnknownInteraction { interaction, .. } => {
            tracing::warn!("received unknown interaction \"{}\"", interaction.data.name);
        }
        poise::FrameworkError::__NonExhaustive(unreachable) => match unreachable {},
    }

    Ok(())
}
