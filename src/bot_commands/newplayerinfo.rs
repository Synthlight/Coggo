#[command("newplayerinfo")]
async fn new_player_info(ctx: &Context, msg: &Message) -> CommandResult {
    if !DEBUG {
        return Ok(());
    }

    if let Err(why) = msg.channel_id.say(ctx, format!("Yes. Though you've come to a biased place to ask.
For more info:
Pros'n'cons: https://discord.com/channels/444244464903651348/445199967540346881/801203088806641704
Watch things in <#{}> to get a feel for it.", VIDEOS_AND_STREAMS)).await {
        println!("Error sending message: '{:?}'.", why);
        return Ok(());
    };

    lib::delete_self(ctx, msg).await;
    return Ok(());
}