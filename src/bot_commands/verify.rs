#[command]
async fn verify(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(ctx, |m| {
        m.content("Try verifying the game files first; here's how:");
        m.embed(|e| {
            e.image("https://cdn.discordapp.com/attachments/589065025138851850/804820863534497822/unknown.png")
        })
    }).await {
        println!("Error sending message: '{:?}'.", why);
        return Ok(());
    };

    return Ok(());
}