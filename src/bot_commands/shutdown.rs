#[command("baddog")]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = msg.author.id.0;

    if (user_id == LORD_GREGORY && msg.guild_id.unwrap().0 == VOLCANOIDS) || (msg.channel_id.0 == ADMIN_BOT_CHAT_VOLC && user_id == HAB) {
        msg.channel_id.say(ctx, "Shutting down.").await.expect("Error sending message.");

        let data = ctx.data.read().await;
        let manager = data.get::<ShardManagerContainer>().unwrap();
        manager.lock().await.shutdown_all().await;
    }

    return Ok(());
}