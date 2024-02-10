#[command]
async fn verify(ctx: &Context, msg: &Message) -> CommandResult {
    if !should_run_on_target_server(msg) {
        return Ok(());
    }

    msg.channel_id.send_message(ctx, CreateMessage::new()
        .content("Try verifying the game files first; here's how:")
        .embed(CreateEmbed::new().image("https://cdn.discordapp.com/attachments/589065025138851850/804820863534497822/unknown.png")),
    ).await.expect("Error sending message.");

    return Ok(());
}