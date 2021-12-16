#[command("baddog")]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.0 != 425347484869591050 { // LordGregory's Id.
        return Ok(());
    }

    msg.channel_id.say(ctx, "Shutting down.").await.expect("Error sending message.");

    let data = ctx.data.read().await;
    let manager = data.get::<ShardManagerContainer>().unwrap();
    manager.lock().await.shutdown_all().await;

    return Ok(());
}