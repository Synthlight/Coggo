// The built-in implementation (derived from re2 I believe) doesn't support it.
// Doesn't work right with fancy-regex either.
const DISABLE_QUOTE_LOOKAHEAD: bool = true;

// ( [^ \\n]+?)             Match a single word.
// ([^\\.\\n]*)             Matches everything except period & newline.
// ( (?!only)[^ \\n]+?)     Match any single word except `only`.
// (Don't forget the `\n` in `[^ \n]` else it actually matches past newlines.)

const CONSOLE_PART1: &str = "(will|game|to|available)";
const CONSOLE_PART2: &str = "(console|xbox|ps4|ps5|playstation)";

lazy_static! {
    static ref CONSOLE_AUTOREPLY_REGEX: Regex = {
        create_auto_reply_regex(&[
            format!("{}.*{}", CONSOLE_PART1, CONSOLE_PART2),
            format!("{}.*{}", CONSOLE_PART2, CONSOLE_PART1)
        ], true)
    };
}

// A var since I keep copying the "the game", "it", "this", etc in many of these.
const THE_GAME_PART1: &str = "(that|the|this)"; // The 'the' part of 'the game'. The group of words that match the first part.

const THE_GAME_PART2: &str = "(game|it|volcanoid(s?))"; // The 'game' part of 'the game'. The group of words that match the last part.

// Merge so we either match: The first part, the second part, or both parts.
// e.g. we match: 'the', 'game', or 'the game'.
const THE_GAME_REGEX: &str = formatcp!("({the}|{game}|{the} {game})", the = THE_GAME_PART1, game = THE_GAME_PART2);

lazy_static! {
    static ref STEAM_AUTOREPLY_REGEX: Regex = {
        create_auto_reply_regex(&[
            format!("when(('|’)s|s| is)?( {})? (come|coming) out", THE_GAME_REGEX),
            format!("is {} (out|released|available)( yet)?", THE_GAME_REGEX),
            format!("is {} (up|available) (yet|to download)?", THE_GAME_REGEX),
            format!("(where|how) (can|do|does)( [^ \\n]+?)? (get|buy|play) (this|it|{} {})", THE_GAME_PART1, THE_GAME_PART2),
            format!("(where|how).*?download"),
            format!("(is|if|will)( [^ \\n]+?)? {}( (?!only)[^ \\n]+?)? (free|on steam)", THE_GAME_REGEX),
            format!("what.*?(get|buy|is)( [^ \\n]+?)? {}.*? on[^a-zA-Z]", THE_GAME_PART2),
            format!("how mu(t?)ch .*?{}? cost", THE_GAME_REGEX),
            format!("how (much|many)( [^ \\n]+?)? is {}", THE_GAME_REGEX),
            format!("can i play( [^ \\n]+?)?( {})? now", THE_GAME_REGEX),
            format!("price in (usd|dollars|aud|cad)")
        ], true)
    };
}

const MULTIPLAYER_NAMES: &str = "(coop|co-op|multiplayer|multi player|multi-player)";

lazy_static! {
    static ref MULTIPLAYER_AUTOREPLY_REGEX: Regex = {
        create_auto_reply_regex(&[
            format!("is {} {}", THE_GAME_REGEX, MULTIPLAYER_NAMES),
            format!("is there {}", MULTIPLAYER_NAMES),
            format!("does {}.*{}", THE_GAME_REGEX, MULTIPLAYER_NAMES),
            format!("{} .* (is )?{}\\?", THE_GAME_REGEX, MULTIPLAYER_NAMES),
            format!("is {} a thing", MULTIPLAYER_NAMES),
            format!("{} is {}.*?\\?", THE_GAME_REGEX, MULTIPLAYER_NAMES),
            format!("you should[^\\.\\n]*(add|make)[^\\.\\n]*{}", MULTIPLAYER_NAMES)
        ], true)
    };
}

lazy_static! {
    static ref STEAM_SCAM: Regex = {
        create_auto_reply_regex(&[
            format!("\\/t[a-zA-Z]+?r\\/new\\/"),
            format!("\\/new\\/\\?partner=")
        ], true)
    };
}

lazy_static! {
    static ref STEAM_SCAM_IGNORE: Regex = {
        create_auto_reply_regex(&[
            format!("https?:\\/\\/(?:www\\.)?steamcommunity.com")
        ], true)
    };
}

struct Info<'a> {
    ctx: &'a Context,
    channel_id: &'a ChannelId,
    thumbs_up: &'a Emoji,
    thumbs_down: &'a Emoji,
}

#[hook]
async fn auto_reply(ctx: &Context, msg: &Message) {
    setup_emoji(ctx).await;

    if !should_run_on_target_server(msg) {
        return;
    }

    let guild = msg.guild_id.unwrap();
    let guild_id = guild.as_u64();
    let channel_id = msg.channel_id.as_u64();

    if msg.content == "Is it on console?" && channel_id == &SECRET_SECTOR {
        msg.channel_id.say(ctx, "No it is not, stop asking.").await.expect("Error sending message.");
        return;
    } else if channel_id == &SECRET_SECTOR {
        return;
    }

    let info = Info {
        ctx,
        channel_id: &msg.channel_id,
        thumbs_up: &get_thumbs_up(guild_id),
        thumbs_down: &get_thumbs_down(guild_id),
    };

    let is_on_debug_server = DEBUG.load(Ordering::Relaxed) && (guild_id == &COGGO_TESTING || guild_id == &CAPS_SUB);
    let should_run_on_volcanoids = !DEBUG.load(Ordering::Relaxed) && guild_id == &VOLCANOIDS;

    // Auto-reply for "console" & "steam". (For Volcanoids, ignore #discuss-other-games.)
    if is_on_debug_server || (should_run_on_volcanoids && channel_id != &DISCUSS_OTHER_GAMES) {
        if CONSOLE_AUTOREPLY_REGEX.is_match(&msg.content).unwrap() {
            create_auto_reply(&info, "**Volcanoids**? On **consoles**? Yes sir! But so far the main priority is adding more content before they dive into all the console shenanigans. That Rich guy will keep you updated!", true).await;
        }
        if STEAM_AUTOREPLY_REGEX.is_match(&msg.content).unwrap() {
            create_auto_reply(&info, "You can get Volcanoids on Steam here: https://store.steampowered.com/app/951440/Volcanoids?utm_source=discord&utm_medium=autoreply", true).await;
        }
    }

    // Auto-reply for "multiplayer". (For Volcanoids, only run in #new-tunnelers, #discussion & #ask-the-community.)
    if is_on_debug_server || (should_run_on_volcanoids && (channel_id == &NEW_TUNNELERS || channel_id == &DISCUSSION || channel_id == &ASK_THE_COMMUNITY || channel_id == &ADMIN_BOT_CHAT_VOLC)) {
        if MULTIPLAYER_AUTOREPLY_REGEX.is_match(&msg.content).unwrap() {
            create_auto_reply(&info, formatcp!("Yes! Volcanoids is multiplayer. See the <#{}> for details.", FAQ), true).await;
        }
    }

    if is_on_debug_server || should_run_on_volcanoids {
        if STEAM_SCAM.is_match(&msg.content).unwrap() && !STEAM_SCAM_IGNORE.is_match(&msg.content).unwrap() {
            quarantine_message(&info, msg).await;
        }
    }
}

async fn create_auto_reply<'a>(info: &'a Info<'a>, text: &str, include_check_faq_msg_in_response: bool) {
    let mut response = text.to_string();
    if include_check_faq_msg_in_response == true {
        response += &format!("\n\nIf you have any other questions, make sure to read the <#{}>, your question might be already answered there.", FAQ);
    }
    let response_with_react_info = response.clone() + &format!("\n\nThis autoreply is a work in progress feature, did this help you? (react with {}) Or was it misplaced? (react with {}) Thanks for the input!", info.thumbs_up, info.thumbs_down);

    let thumbs_up_reaction = ReactionType::from(info.thumbs_up.clone());
    let thumbs_down_reaction = ReactionType::from(info.thumbs_down.clone());

    // Create msg & reactions.
    let mut msg = info.channel_id.say(info.ctx, response_with_react_info).await.expect("Error sending auto-reply.");
    msg.react(info.ctx, thumbs_up_reaction.clone()).await.expect("Error adding thumbs up emoji.");
    msg.react(info.ctx, thumbs_down_reaction.clone()).await.expect("Error adding thumbs down emoji.");

    // Wait for the reaction, filtered to match the above two.
    let thumbs_up_reaction_copy = thumbs_up_reaction.clone();
    let thumbs_down_reaction_copy = thumbs_down_reaction.clone();
    let reaction = msg.await_reaction(info.ctx)
        .filter(move |f| { f.emoji == thumbs_up_reaction_copy || f.emoji == thumbs_down_reaction_copy })
        .timeout(Duration::from_secs(10))
        .await;

    // The user didn't respond in time, delete the auto-reply.
    if reaction.is_none() {
        msg.delete(info.ctx).await.expect("Error deleting auto-reply message.");
        return;
    };

    // The user did respond. Whether positive or negative, edit out the feedback part and remove all reactions.
    msg.edit(info.ctx, |m| m.content(response)).await.expect("Error editing auto-reply message.");
    msg.delete_reaction_emoji(info.ctx, thumbs_up_reaction.clone()).await.expect("Error deleting auto-reply reactions.");
    msg.delete_reaction_emoji(info.ctx, thumbs_down_reaction.clone()).await.expect("Error deleting auto-reply reactions.");

    // Leave a thanks for the feedback msg for {n} seconds.
    let feedback_msg = msg.channel_id.say(info.ctx, "Thanks for the feedback").await.expect("Error sending thanks for the feedback message.");
    sleep(Duration::from_secs(5)).await;
    // ...and then delete it.
    feedback_msg.delete(info.ctx).await.expect("Error deleting thanks for the feedback message.");

    // If the user gave a thumbs down, delete the auto-reply.
    if reaction.unwrap().as_inner_ref().emoji == thumbs_down_reaction {
        msg.delete(info.ctx).await.expect("Error deleting auto-reply message.");
    }
}

async fn quarantine_message<'a>(info: &'a Info<'a>, msg: &Message) {
    let guild = msg.guild_id.unwrap();
    let guild_id = guild.as_u64();
    let channel_id: u64;

    if guild_id == &COGGO_TESTING {
        channel_id = ADMIN_BOT_CHAT_TEST
    } else if guild_id == &VOLCANOIDS {
        channel_id = ADMIN_BOT_CHAT_VOLC
    } else {
        return;
    }

    let admin_bot_channel = ChannelId::from(channel_id);

    msg.delete(info.ctx).await.expect("Error deleting scam message.");

    let mut report = admin_bot_channel.say(info.ctx, format!("Deleted potential scam message in <#{}> by <@{}>\n{}", msg.channel_id, msg.author.id, msg.content)).await.expect("Error reporting scam message.");
    report.suppress_embeds(info.ctx).await.expect("Error removing embeds.");
}

fn create_auto_reply_regex(individual_lines_to_match: &[String], ignore_quoted_text: bool) -> Regex {
    let mut regex_str = String::new();

    for (index, line) in individual_lines_to_match.iter().enumerate() {
        let mut to_match = line.to_string();
        if index > 0 {
            regex_str += "|";
        }
        // Adds a prefix to each group that negates msgs starting with `>` (quotes).
        if ignore_quoted_text == true && !DISABLE_QUOTE_LOOKAHEAD {
            to_match += &format!(r"^(?!>).*?{}", to_match);
        }
        regex_str += &format!("(?:{})", to_match);
    }

    if DEBUG.load(Ordering::Relaxed) {
        println!("Made regex: {}", regex_str);
    }

    // `(?im)` is used for `fancy-regex`. For `regex`, uncomment the ca
    return RegexBuilder::new(&format!("(?im){}", &regex_str))
        //.case_insensitive(true) // `i`
        //.multi_line(true) // `m`
        .build()
        .unwrap();
}

fn get_thumbs_up(guild_id: &u64) -> Emoji {
    unsafe {
        if guild_id == &COGGO_TESTING {
            return THUMBS_UP_TESTING.clone().unwrap();
        } else if guild_id == &CAPS_SUB {
            return THUMBS_UP_KAPPA.clone().unwrap();
        } else if guild_id == &VOLCANOIDS {
            return THUMBS_UP_COG_HAND.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }
}

fn get_thumbs_down(guild_id: &u64) -> Emoji {
    unsafe {
        if guild_id == &COGGO_TESTING {
            return THUMBS_DOWN_TESTING.clone().unwrap();
        } else if guild_id == &CAPS_SUB {
            return THUMBS_DOWN_SHOTGUN.clone().unwrap();
        } else if guild_id == &VOLCANOIDS {
            return THUMBS_DOWN_COG_HAND.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }
}