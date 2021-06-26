#[command("uptime")]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let time_diff = Local::now() - START_TIME.clone();
    let days = time_diff.num_days() % 24;
    let hours = time_diff.num_hours() % 60;
    let minutes = time_diff.num_minutes() % 60;
    let seconds = time_diff.num_seconds() % 60;
    let text: String;

    if days > 0 {
        text = format!("Uptime: {} days, {} hours, {} minutes, {} seconds.", days, hours, minutes, seconds);
    } else if hours > 0 {
        text = format!("Uptime: {} hours, {} minutes, {} seconds.", hours, minutes, seconds);
    } else if minutes > 0 {
        text = format!("Uptime: {} minutes, {} seconds.", minutes, seconds);
    } else {
        text = format!("Uptime: {} seconds.", seconds);
    }

    msg.channel_id.say(ctx, text).await.expect("Error sending message.");

    return Ok(());
}