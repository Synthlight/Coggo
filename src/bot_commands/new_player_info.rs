#[command("newplayerinfo")]
async fn new_player_info(ctx: &Context, msg: &Message) -> CommandResult {
    if !should_run_on_target_server(msg) {
        return Ok(());
    }

    msg.channel_id.say(ctx, format!("Yes. Though you've come to a biased place to ask.
For more info:
Pros'n'cons: https://discord.com/channels/444244464903651348/445199967540346881/801203088806641704
Watch things in <#{}> to get a feel for it.", VIDEOS_AND_STREAMS)).await.expect("Error sending message.");

    return Ok(());
}