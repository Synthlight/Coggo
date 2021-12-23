use std::env;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_std::sync::Mutex;
use async_std::task::sleep;
use chrono::{DateTime, Local};
use const_format::formatcp;
use fancy_regex::{Regex, RegexBuilder};
use once_cell::sync::Lazy;
use serde_json::Value;
use serenity::async_trait;
use serenity::builder::EditMessage;
use serenity::client::{Client, Context, EventHandler};
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::{CommandError, CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group, hook};
use serenity::http::CacheHttp;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::guild::Emoji;
use serenity::model::id::{ChannelId, EmojiId, GuildId, MessageId};
use serenity::prelude::TypeMapKey;
use serenity::utils::hashmap_to_json_map;

include!["lib/lib.rs"];

include!["models/consts.rs"];
include!["models/emoji.rs"];
include!["models/spam_list.rs"];

include!["auto_reply/auto_reply.rs"];

include!["bot_commands/help.rs"];
include!["bot_commands/how_to_paint.rs"];
include!["bot_commands/new_player_info.rs"];
include!["bot_commands/no.rs"];
include!["bot_commands/shutdown.rs"];
include!["bot_commands/uptime.rs"];
include!["bot_commands/verify.rs"];

static DEBUG: AtomicBool = AtomicBool::new(true);
static START_TIME: Lazy<RwLock<DateTime<Local>>> = Lazy::new(|| RwLock::new(Local::now()));
static EMOJI: Lazy<Arc<Mutex<CachedEmoji>>> = Lazy::new(|| Arc::new(Mutex::new(CachedEmoji::new())));
static SPAM_LIST: Lazy<Arc<Mutex<SpamList>>> = Lazy::new(|| Arc::new(Mutex::new(SpamList::new())));

#[group]
#[commands(help, how_to_paint, new_player_info, no, shutdown, uptime, verify)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ready(&ctx).await;
    }
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<serenity::prelude::Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    let debug_str = env::var("coggo_debug").unwrap_or("true".to_string());
    let is_debug = bool::from_str(&debug_str).expect("Error parsing `coggo_debug` value. Must be true/false.");
    DEBUG.store(is_debug, Ordering::Relaxed);

    println!("Start time: {}", START_TIME.read().unwrap().format("%Y-%m-%d -- %H:%M:%S"));

    if is_debug {
        println!("Debug mode enabled.");
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".")
            .allow_dm(false))
        .group(&GENERAL_GROUP)
        .normal_message(auto_reply)
        .after(after_hook);

    let mut client = Client::builder(env::var("token").expect("You must pass a token as `token` env var."))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    // Start listening for events by starting a single shard.
    client.start().await.expect("An error occurred while running the client.");
}

async fn ready(ctx: &Context) {
    EMOJI.lock().await.setup(&ctx).await;
    SPAM_LIST.lock().await.setup().await;

    prep_regex();

    println!("Startup complete.");
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    match error {
        Ok(()) => println!("Processed command '{}'.", cmd_name),
        Err(why) => println!("Command '{}' returned error '{:?}'.", cmd_name, why),
    }
}