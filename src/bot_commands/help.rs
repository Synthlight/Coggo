#[command("chelp")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if !should_run_on_target_server(msg) {
        return Ok(());
    }

    msg.channel_id.send_message(ctx, |m| {
        m.allowed_mentions(|f| {f.empty_users()});
        m.embed(|e| {
            e.field(".chelp", "You're lookin' at it.", false);
            e.field(".uptime", "Prints bot uptime.", false);
            e.field(".verify", "Prints info about how to verify Steam game files.", false);
            e.field(".baddog", format!("Shuts down the bot. Only usable by <@{}> or <@{}> and only in <#{}>. MUST BE RESTARTED MANUALLY. Use only as a last resort.", HAB, LORD_GREGORY, ADMIN_BOT_CHAT_VOLC), false)
        })
    }).await.expect("Error sending message.");

    return Ok(());
}