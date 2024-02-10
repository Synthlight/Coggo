use serenity::all::Message;
use serenity::builder::EditMessage;

use crate::macros::non_zero_u64;
use crate::models::consts::*;

fn should_run_on_target_server(msg: &Message) -> bool {
    if msg.guild_id.is_none() {
        return false;
    }

    let guild_id = non_zero_u64!(msg.guild_id.unwrap().get());

    let is_debug = DEBUG.load(Ordering::Relaxed);
    let run_on_volc = !is_debug && guild_id == VOLCANOIDS;
    let run_on_testing = is_debug && (guild_id == COGGO_TESTING || guild_id == CAPS_SUB);

    return run_on_volc || (run_on_testing);
}

async fn edit_msg_text(ctx: &Context, msg: &mut Message, new_text: &String) -> serenity::Result<()> {
    let mut builder = EditMessage::default();
    builder = builder.content(new_text);
    return msg.edit(ctx, builder).await;

    //let map = hashmap_to_json_map(builder.0);
    //return ctx.http().edit_message(msg.channel_id, msg.id, &Value::Object(map), vec![]).await;
}