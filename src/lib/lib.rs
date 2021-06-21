// pub async fn delete_self(ctx: &Context, msg: &Message) -> bool {
//     if let Err(why) = msg.delete(ctx).await {
//         println!("Error deleting message '{:?}'.", why);
//         return false;
//     };
//
//     return true;
// }

pub fn should_run_on_target_server(msg: &Message) -> bool {
    if msg.guild_id.is_none() {
        return false;
    }
    let guild = msg.guild_id.unwrap();
    let guild_id = guild.as_u64();

    return !DEBUG.load(Ordering::Relaxed) && guild_id == &VOLCANOIDS
        || (DEBUG.load(Ordering::Relaxed) && (guild_id == &COGGO_TESTING || guild_id == &CAPS_SUB));
}