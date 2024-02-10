use serenity::all::standard::CommandResult;
use serenity::builder::{CreateAllowedMentions, CreateEmbed, CreateMessage};
use serenity::framework::standard::macros::command;

#[command("chelp")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if !should_run_on_target_server(msg) {
        return Ok(());
    }

    msg.channel_id.send_message(ctx, CreateMessage::new()
        .allowed_mentions(CreateAllowedMentions::new().empty_users())
        .embed(CreateEmbed::new()
            .field(".chelp", "You're lookin' at it.", false)
            .field(".uptime", "Prints bot uptime.", false)
            .field(".verify", "Prints info about how to verify Steam game files.", false)
            .field(".baddog", format!("Shuts down the bot. Only usable by <@{}> or <@{}> and only in <#{}>. MUST BE RESTARTED MANUALLY. Use only as a last resort.", HAB, LORD_GREGORY, ADMIN_BOT_CHAT_VOLC), false)
        ),
    ).await.expect("Error sending message.");

    return Ok(());
}