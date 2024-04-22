use std::num::NonZeroU64;

use serenity::all::{ChannelId, Context, Message};

use crate::models::consts::*;
use crate::util::lib::*;
use crate::util::macros::*;

pub async fn teaser_detection(ctx: &Context, msg: &Message) {
    if true { return; }

    if !should_run_on_target_server(msg) {
        return;
    }

    let guild_id = non_zero_u64!(msg.guild_id.unwrap().into());
    let channel_id = non_zero_u64!(msg.channel_id.into());
    let user_id = non_zero_u64!(msg.author.id.into());

    if channel_id != DISCUSSION && channel_id != GENERAL { return; }

    let report_channel_id: NonZeroU64;
    if guild_id == COGGO_TESTING {
        report_channel_id = ADMIN_BOT_CHAT_TEST_TEASER_THREAD
    } else if guild_id == VOLCANOIDS {
        report_channel_id = ADMIN_BOT_CHAT_TEASER_THREAD
    } else {
        return;
    }

    let report_channel = ChannelId::from(report_channel_id);
    let msg_contents = msg.content.to_lowercase();

    if msg_contents.contains("cup") || msg_contents.contains("mug") {
        report_channel.say(ctx, format!("Found message that matches cup/mug in <#{}> by <@{}>\n{}\n{}", channel_id.get(), user_id.get(), msg.link(), msg_contents)).await.expect("Error sending message.");
    }
}