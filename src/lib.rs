use serenity::client::{Context};
use serenity::model::channel::Message;

pub async fn delete_self(ctx: &Context, msg: &Message) -> bool {
    if let Err(why) = msg.delete(ctx).await {
        println!("Error deleting message '{:?}'.", why);
        return false;
    };

    return true;
}