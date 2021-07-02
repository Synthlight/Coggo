fn should_run_on_target_server(msg: &Message) -> bool {
    if msg.guild_id.is_none() {
        return false;
    }
    let guild_id = msg.guild_id.unwrap().0;

    return !DEBUG.load(Ordering::Relaxed) && guild_id == VOLCANOIDS
        || (DEBUG.load(Ordering::Relaxed) && (guild_id == COGGO_TESTING || guild_id == CAPS_SUB));
}

async fn edit_msg_text(ctx: &Context, msg: &Message, new_text: &String) -> serenity::Result<Message> {
    let mut builder = EditMessage::default();
    builder.content(new_text);
    let map = hashmap_to_json_map(builder.0);

    return ctx.http().edit_message(msg.channel_id.0, msg.id.0, &Value::Object(map)).await;
}