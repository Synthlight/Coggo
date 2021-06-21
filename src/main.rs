use std::env;
use std::time::Duration;

use async_std::task::sleep;
use const_format::formatcp;
use fancy_regex::{Regex, RegexBuilder};
use lazy_static::lazy_static;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{CommandError, CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group, hook};
use serenity::model::channel::{Message, ReactionType};
use serenity::model::guild::Emoji;
use serenity::model::id::{ChannelId, EmojiId, GuildId};

const DEBUG: bool = true;

const COGGO_TESTING: u64 = 853358073964265512;
const VOLCANOIDS: u64 = 444244464903651348;
const CAPS_SUB: u64 = 488708757304639520;

const VIDEOS_AND_STREAMS: u64 = 450765632577863691;
const FAQ: u64 = 454972890299891723;
const NEW_TUNNELERS: u64 = 463721722638499850;
const DISCUSSION: u64 = 445199967540346881;
const ASK_THE_COMMUNITY: u64 = 494576341849735188;
const DISCUSS_OTHER_GAMES: u64 = 496325967883534337;
const SECRET_SECTOR: u64 = 500781638075154468;
const ADMIN_BOT_CHAT: u64 = 523288694514515969;

// Coggo-Testing
const THUMBS_UP_ID_TESTING: u64 = 853850623576768578;
const THUMBS_DOWN_ID_TESTING: u64 = 853850623392088074;

// Cap's Sub
const THUMBS_UP_ID_KAPPA: u64 = 545279802198851615;
const THUMBS_DOWN_ID_SHOTGUN: u64 = 546734308161749011;

// Volcanoids
const THUMBS_UP_ID_COG_HAND: u64 = 713469848193073303;
const THUMBS_DOWN_ID_COG_HAND: u64 = 722120016723574805;

static mut THUMBS_UP_TESTING: Option<Emoji> = None;
static mut THUMBS_UP_COG_HAND: Option<Emoji> = None;
static mut THUMBS_UP_KAPPA: Option<Emoji> = None;
static mut THUMBS_DOWN_TESTING: Option<Emoji> = None;
static mut THUMBS_DOWN_COG_HAND: Option<Emoji> = None;
static mut THUMBS_DOWN_SHOTGUN: Option<Emoji> = None;

include!["lib/lib.rs"];

#[group]
#[commands(verify, new_player_info, no)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP)
        .normal_message(auto_reply)
        .before(before_hook)
        .after(after_hook);

    let mut client = Client::builder(env::var("token").expect("You must pass a token as `token` env var."))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    if !should_run_on_target_server(msg) {
        return false;
    }

    println!("Running command {}.", cmd_name);

    setup_emoji(ctx).await;

    true
}

async fn setup_emoji(ctx: &Context) {
    unsafe {
        if THUMBS_UP_TESTING.is_none() {
            println!("Fetching/caching emojis.");

            THUMBS_UP_TESTING = Some(GuildId(COGGO_TESTING).emoji(ctx, EmojiId(THUMBS_UP_ID_TESTING)).await.unwrap());
            THUMBS_UP_COG_HAND = Some(GuildId(VOLCANOIDS).emoji(ctx, EmojiId(THUMBS_UP_ID_COG_HAND)).await.unwrap());
            THUMBS_UP_KAPPA = Some(GuildId(CAPS_SUB).emoji(ctx, EmojiId(THUMBS_UP_ID_KAPPA)).await.unwrap());
            THUMBS_DOWN_TESTING = Some(GuildId(COGGO_TESTING).emoji(ctx, EmojiId(THUMBS_DOWN_ID_TESTING)).await.unwrap());
            THUMBS_DOWN_COG_HAND = Some(GuildId(VOLCANOIDS).emoji(ctx, EmojiId(THUMBS_DOWN_ID_COG_HAND)).await.unwrap());
            THUMBS_DOWN_SHOTGUN = Some(GuildId(CAPS_SUB).emoji(ctx, EmojiId(THUMBS_DOWN_ID_SHOTGUN)).await.unwrap());
        }
    }
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    match error {
        Ok(()) => println!("Processed command '{}'.", cmd_name),
        Err(why) => println!("Command '{}' returned error '{:?}'.", cmd_name, why),
    }
}

include!["auto_reply/auto_reply.rs"];
include!["bot_commands/newplayerinfo.rs"];
include!["bot_commands/no.rs"];
include!["bot_commands/verify.rs"];