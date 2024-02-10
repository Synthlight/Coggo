use std::env;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

use async_std::sync::Mutex;
use chrono::{DateTime, Local};
use once_cell::sync::Lazy;
use serenity::all::{Context, EventHandler, GatewayIntents, Message, Ready, ShardManager, StandardFramework};
use serenity::all::standard::{CommandError, Configuration};
use serenity::all::standard::macros::{group, hook};
use serenity::{async_trait, Client};
use serenity::prelude::TypeMapKey;

use crate::auto_reply::*;
use crate::bot_commands::help::*;
use crate::bot_commands::no::*;
use crate::bot_commands::shutdown::*;
use crate::bot_commands::uptime::*;
use crate::bot_commands::verify::*;
use crate::models::emoji::*;
use crate::models::spam_list::*;

pub mod auto_reply;
pub mod bot_commands;
pub mod models;
pub mod util;

static DEBUG: AtomicBool = AtomicBool::new(true);
static START_TIME: Lazy<RwLock<DateTime<Local>>> = Lazy::new(|| RwLock::new(Local::now()));
static EMOJI: Lazy<Arc<Mutex<CachedEmoji>>> = Lazy::new(|| Arc::new(Mutex::new(CachedEmoji::new())));
static SPAM_LIST: Lazy<Arc<Mutex<SpamList>>> = Lazy::new(|| Arc::new(Mutex::new(SpamList::new())));

#[group]
#[commands(help, no, shutdown, uptime, verify)]
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
    type Value = Arc<ShardManager>;
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
        .group(&GENERAL_GROUP)
        .normal_message(message_hook)
        .after(after_hook);
    framework.configure(Configuration::new()
        .prefix(".")
        .allow_dm(false));

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(env::var("token").expect("You must pass a token as `token` env var."), intents)
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
pub async fn message_hook(ctx: &Context, msg: &Message) {
    auto_reply(ctx, msg).await;
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    match error {
        Ok(()) => println!("Processed command '{}'.", cmd_name),
        Err(why) => println!("Command '{}' returned error '{:?}'.", cmd_name, why),
    }
}