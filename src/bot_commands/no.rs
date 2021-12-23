#[command]
async fn no(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.0 != 425347484869591050 { // LordGregory's Id.
        return Ok(());
    }

    msg.channel_id.say(ctx, "No, now stop asking.").await.expect("Error sending message.");

    return Ok(());
}