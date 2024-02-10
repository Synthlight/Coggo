use serenity::all::{Context, Message};
use serenity::all::standard::CommandResult;
use serenity::all::standard::macros::command;

#[command]
async fn no(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.get() != 425347484869591050 { // LordGregory's Id.
        return Ok(());
    }

    msg.channel_id.say(ctx, "No, now stop asking.").await.expect("Error sending message.");

    return Ok(());
}