use std::{any, sync::Arc};

use serenity::{model::{prelude::{GuildId, UserId}, user::User}, prelude::Context, Error, http::Http};

pub async fn kick(guild: GuildId, http: &Arc<Http>, user: UserId, reason: &str) -> Result<(), Error> {
    guild.kick_with_reason(http, UserId::from(user), reason).await?;

    Ok(())
}
