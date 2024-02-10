use std::num::NonZeroU64;

use serenity::all::{Context, Message};
use serenity::all::standard::CommandResult;
use serenity::all::standard::macros::command;

use crate::models::consts::*;
use crate::util::macros::*;
use crate::ShardManagerContainer;

#[command("baddog")]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = non_zero_u64!(msg.author.id.get());
    let guild_id = non_zero_u64!(msg.guild_id.unwrap().get());
    let channel_id = non_zero_u64!(msg.channel_id.get());

    if (user_id == LORD_GREGORY && guild_id == VOLCANOIDS) || (channel_id == ADMIN_BOT_CHAT_VOLC && user_id == HAB) {
        msg.channel_id.say(ctx, "Shutting down.").await.expect("Error sending message.");

        let data = ctx.data.read().await;
        let manager = data.get::<ShardManagerContainer>().unwrap();
        manager.shutdown_all().await;
    }

    return Ok(());
}