fn should_run_on_target_server(msg: &Message) -> bool {
    if msg.guild_id.is_none() {
        return false;
    }
    let guild_id = msg.guild_id.unwrap().0;

    let is_debug = DEBUG.load(Ordering::Relaxed);
    let run_on_volc = !is_debug && guild_id == VOLCANOIDS;
    let run_on_testing = is_debug && (guild_id == COGGO_TESTING || guild_id == CAPS_SUB);

    return run_on_volc || (run_on_testing);
}

async fn edit_msg_text(ctx: &Context, msg: &Message, new_text: &String) -> serenity::Result<Message> {
    let mut builder = EditMessage::default();
    builder.content(new_text);
    let map = hashmap_to_json_map(builder.0);

    return ctx.http().edit_message(msg.channel_id.0, msg.id.0, &Value::Object(map)).await;
}