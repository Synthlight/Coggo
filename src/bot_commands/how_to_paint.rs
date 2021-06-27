#[command("howtopaint")]
async fn how_to_paint(ctx: &Context, msg: &Message) -> CommandResult {
    if !should_run_on_target_server(msg) {
        return Ok(());
    }

    msg.channel_id.say(ctx, "Here is a video tutorial: <https://www.youtube.com/watch?v=ZMOXq__oIBw>
And here is a steam guide: <https://steamcommunity.com/sharedfiles/filedetails/?id=2489200340>").await.expect("Error sending message.");

    return Ok(());
}