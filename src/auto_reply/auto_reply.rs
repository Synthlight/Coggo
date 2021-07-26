// The built-in implementation (derived from re2 I believe) doesn't support it.
// Doesn't work right with fancy-regex either.
const DISABLE_QUOTE_LOOKAHEAD: bool = true;

// ( [^ \\n]+?)             Match a single word.
// ([^\\.\\n]*)             Matches everything except period & newline.
// ( (?!only)[^ \\n]+?)     Match any single word except `only`.
// (Don't forget the `\n` in `[^ \n]` else it actually matches past newlines.)

const CONSOLE_PART1: &str = r"(will|game|to|available)";
const CONSOLE_PART2: &str = r"(console|xbox|ps4|ps5|playstation)";

static CONSOLE_AUTO_REPLY_REGEX: Lazy<RwLock<Regex>> = Lazy::new(|| RwLock::new(create_auto_reply_regex(&[
    format!(r"{}.*{}", CONSOLE_PART1, CONSOLE_PART2),
    format!(r"{}.*{}", CONSOLE_PART2, CONSOLE_PART1)
], true)));

// A var since I keep copying the "the game", "it", "this", etc in many of these.
const THE_GAME_PART1: &str = r"(that|the|this)";
const THE_GAME_PART2: &str = r"(game|it|volcanoid(s?))";

// Merge so we either match: The first part, the second part, or both parts.
// e.g. we match: 'the', 'game', or 'the game'.
const THE_GAME_REGEX: &str = formatcp!(r"({the}|{game}|{the} {game})", the = THE_GAME_PART1, game = THE_GAME_PART2);

static STEAM_AUTO_REPLY_REGEX: Lazy<RwLock<Regex>> = Lazy::new(|| RwLock::new(create_auto_reply_regex(&[
    format!(r"when(('|’)s|s| is)?( {})? (come|coming) out", THE_GAME_REGEX),
    format!(r"is {} (out|released|available)( yet)?", THE_GAME_REGEX),
    format!(r"is {} (up|available) (yet|to download)?", THE_GAME_REGEX),
    format!(r"(where|how) (can|do|does)( [^ \n]+?)? (get|buy|play) (this|it|{} {})", THE_GAME_PART1, THE_GAME_PART2),
    format!(r"(where|how).*?download"),
    format!(r"(is|if|will)( [^ \n]+?)? {}( (?!only)[^ \n]+?)? (free|on steam)", THE_GAME_REGEX),
    format!(r"what.*?(get|buy|is)( [^ \n]+?)? {}.*? on[^a-zA-Z]", THE_GAME_PART2),
    format!(r"how mu(t?)ch .*?{}? cost", THE_GAME_REGEX),
    format!(r"how (much|many)( [^ \n]+?)? is {}", THE_GAME_REGEX),
    format!(r"can i play( [^ \n]+?)?( {})? now", THE_GAME_REGEX),
    format!(r"price in (usd|dollars|aud|cad)")
], true)));

const MULTIPLAYER_NAMES: &str = r"(coop|co-op|multiplayer|multi player|multi-player)";

static MULTIPLAYER_AUTO_REPLY_REGEX: Lazy<RwLock<Regex>> = Lazy::new(|| RwLock::new(create_auto_reply_regex(&[
    format!(r"is {} {}", THE_GAME_REGEX, MULTIPLAYER_NAMES),
    format!(r"is there {}", MULTIPLAYER_NAMES),
    format!(r"does {}.*{}", THE_GAME_REGEX, MULTIPLAYER_NAMES),
    format!(r"{} .* (is )?{}\?", THE_GAME_REGEX, MULTIPLAYER_NAMES),
    format!(r"is {} a thing", MULTIPLAYER_NAMES),
    format!(r"{} is {}.*?\?", THE_GAME_REGEX, MULTIPLAYER_NAMES),
    format!(r"you should[^\.\n]*(add|make)[^\.\n]*{}", MULTIPLAYER_NAMES)
], true)));

static STEAM_SCAM: Lazy<RwLock<Regex>> = Lazy::new(|| RwLock::new(create_auto_reply_regex(&[
    format!(r"\/t[tradeof]+?[er]\/ne?w\/\?"),
    format!(r"\/n[eo]w\/\?p[partner]+?[er]=")
], true)));

static STEAM_SCAM_IGNORE: Lazy<RwLock<Regex>> = Lazy::new(|| RwLock::new(create_auto_reply_regex(&[
    format!(r"https?:\/\/(?:www\.)?steamcommunity.com")
], true)));

struct Info<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    channel_id: &'a ChannelId,
    thumbs_up: &'a Emoji,
    thumbs_down: &'a Emoji,
}

#[hook]
async fn auto_reply(ctx: &Context, msg: &Message) {
    if !should_run_on_target_server(msg) {
        return;
    }

    let guild_id = msg.guild_id.unwrap().0;
    let channel_id = msg.channel_id.0;

    if msg.content == "Is it on console?" && channel_id == SECRET_SECTOR {
        msg.channel_id.say(ctx, "No it is not, stop asking.").await.expect("Error sending message.");
        return;
    } else if channel_id == SECRET_SECTOR {
        return;
    }

    let thumbs_up = EMOJI.lock().await.get_thumbs_up(guild_id);
    let thumbs_down = EMOJI.lock().await.get_thumbs_down(guild_id);

    let info = Info {
        ctx,
        msg,
        channel_id: &msg.channel_id,
        thumbs_up: &thumbs_up,
        thumbs_down: &thumbs_down,
    };

    let is_on_debug_server = DEBUG.load(Ordering::Relaxed) && (guild_id == COGGO_TESTING || guild_id == CAPS_SUB);
    let should_run_on_volcanoids = !DEBUG.load(Ordering::Relaxed) && guild_id == VOLCANOIDS;

    // Auto-reply for "console" & "steam". (For Volcanoids, ignore #discuss-other-games.)
    if is_on_debug_server || (should_run_on_volcanoids && channel_id != DISCUSS_OTHER_GAMES) {
        if CONSOLE_AUTO_REPLY_REGEX.read().unwrap().is_match(&msg.content).unwrap() {
            create_auto_reply(&info, "**Volcanoids**? On **consoles**? Yes sir! But so far the main priority is adding more content before they dive into all the console shenanigans. That Rich guy will keep you updated!", true).await;
        }
        if STEAM_AUTO_REPLY_REGEX.read().unwrap().is_match(&msg.content).unwrap() {
            create_auto_reply(&info, "You can get Volcanoids on Steam here: https://store.steampowered.com/app/951440/Volcanoids?utm_source=discord&utm_medium=autoreply", true).await;
        }
    }

    // Auto-reply for "multiplayer". (For Volcanoids, only run in #new-tunnelers, #discussion & #ask-the-community.)
    if is_on_debug_server || (should_run_on_volcanoids && (channel_id == NEW_TUNNELERS || channel_id == DISCUSSION || channel_id == ASK_THE_COMMUNITY || channel_id == ADMIN_BOT_CHAT_VOLC)) {
        if MULTIPLAYER_AUTO_REPLY_REGEX.read().unwrap().is_match(&msg.content).unwrap() {
            create_auto_reply(&info, formatcp!("Yes! Volcanoids is multiplayer. See the <#{}> for details.", FAQ), true).await;
        }
    }

    if is_on_debug_server || should_run_on_volcanoids {
        if STEAM_SCAM.read().unwrap().is_match(&msg.content).unwrap() && !STEAM_SCAM_IGNORE.read().unwrap().is_match(&msg.content).unwrap() {
            quarantine_message(&info, msg).await;
        }
    }
}

async fn create_auto_reply<'a>(info: &'a Info<'a>, text: &str, include_check_faq_msg_in_response: bool) {
    println!("User {} triggered auto-reply: {}", info.msg.author.id, text);

    let mut response = text.to_string();
    if include_check_faq_msg_in_response == true {
        response += &format!("\n\nIf you have any other questions, make sure to read the <#{}>, your question might be already answered there.", FAQ);
    }

    let response_with_react_info: String;
    let disable_reactions = info.channel_id.0 == ADMIN_BOT_CHAT_TEST || info.channel_id.0 == ADMIN_BOT_CHAT_VOLC;
    if disable_reactions {
        response_with_react_info = response.clone() + "\n(Reactions disabled for this channel.)";
    } else {
        response_with_react_info = response.clone() + &format!("\n\nThis autoreply is a work in progress feature, did this help you? (react with {}) Or was it misplaced? (react with {}) Thanks for the input!", info.thumbs_up, info.thumbs_down);
    }

    let thumbs_up_reaction = ReactionType::from(info.thumbs_up.clone());
    let thumbs_down_reaction = ReactionType::from(info.thumbs_down.clone());

    // Create msg & reactions.
    let mut msg = info.channel_id.say(info.ctx, response_with_react_info).await.expect("Error sending auto-reply.");

    if disable_reactions {
        return;
    }

    let thumbs_up = msg.react(info.ctx, thumbs_up_reaction.clone()).await.expect("Error adding thumbs up emoji.");
    let thumbs_down = msg.react(info.ctx, thumbs_down_reaction.clone()).await.expect("Error adding thumbs down emoji.");

    // Wait for the reaction, filtered to match the above two.
    let thumbs_up_reaction_copy = thumbs_up_reaction.clone();
    let thumbs_down_reaction_copy = thumbs_down_reaction.clone();
    let reaction = msg.await_reaction(info.ctx)
        .filter(move |f| { f.emoji == thumbs_up_reaction_copy || f.emoji == thumbs_down_reaction_copy })
        .timeout(Duration::from_secs(20))
        .await;

    let mut is_thumbs_up: bool = false;
    let mut is_thumbs_down: bool = false;

    match reaction {
        Some(arc_emoji) => {
            let emoji = &arc_emoji.as_inner_ref().emoji.clone();
            is_thumbs_up = emoji == &thumbs_up_reaction;
            is_thumbs_down = emoji == &thumbs_down_reaction;
        }
        None => {
            // Fetch via msg in case the reaction collector missed the emoji.
            get_thumb_reactions(info, &thumbs_up_reaction, &thumbs_down_reaction, &msg.id, &mut is_thumbs_up, &mut is_thumbs_down).await;
        }
    }

    if is_thumbs_up || is_thumbs_down {
        edit_msg_text(info.ctx, &msg, &response).await.expect("Error editing auto-reply message.");
    }

    thumbs_up.delete_all(info.ctx).await.expect("Error deleting auto-reply reactions.");
    thumbs_down.delete_all(info.ctx).await.expect("Error deleting auto-reply reactions.");

    // The user didn't respond in time, leaving auto-reply note.
    if !(is_thumbs_up || is_thumbs_down) {
        msg.edit(info.ctx, |m| m
            .content("You didn't react... I'll leave this note here so I can fix the potential false positive for next time.")
            .suppress_embeds(true)).await
            .expect("Error editing auto-reply message for final note.");
        println!("User {} didn't react in time, leaving auto-reply note.", info.msg.author.id);
        return;
    };

    // Leave a thanks for the feedback msg for {n} seconds.
    let feedback_msg = msg.channel_id.say(info.ctx, "Thanks for the feedback").await.expect("Error sending thanks for the feedback message.");
    sleep(Duration::from_secs(5)).await;
    // ...and then delete it.
    feedback_msg.delete(info.ctx).await.expect("Error deleting thanks for the feedback message.");

    // The user gave a thumbs down, leaving auto-reply note.
    if is_thumbs_down {
        msg.edit(info.ctx, |m| m
            .content("Thanks for the feedback. I'll leave this note here so I can fix the false positive for next time.")
            .suppress_embeds(true)).await
            .expect("Error editing auto-reply message for final note.");
    }
}

async fn get_thumb_reactions<'a>(info: &'a Info<'a>, thumbs_up_reaction: &ReactionType, thumbs_down_reaction: &ReactionType, msg_id: &MessageId, is_thumbs_up: &mut bool, is_thumbs_down: &mut bool) {
    let msg = info.channel_id.message(info.ctx, MessageId::from(msg_id)).await.unwrap();

    for reaction in &msg.reactions {
        if &reaction.reaction_type == thumbs_up_reaction && reaction.count > 1 {
            *is_thumbs_up = true;
        }
        if &reaction.reaction_type == thumbs_down_reaction && reaction.count > 1 {
            *is_thumbs_down = true;
        }
    }
}

async fn quarantine_message<'a>(info: &'a Info<'a>, msg: &Message) {
    let guild = msg.guild_id.unwrap();
    let guild_id = guild.0;
    let channel_id: u64;

    if guild_id == COGGO_TESTING {
        channel_id = ADMIN_BOT_CHAT_TEST
    } else if guild_id == VOLCANOIDS {
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

    println!("Made regex: {}", regex_str);

    // `(?im)` is used for `fancy-regex`. For `regex`, uncomment the ca
    return RegexBuilder::new(&format!("(?im){}", &regex_str))
        //.case_insensitive(true) // `i`
        //.multi_line(true) // `m`
        .build()
        .unwrap();
}

#[allow(unused)]
fn prep_regex() {
    CONSOLE_AUTO_REPLY_REGEX.read().unwrap();
    STEAM_AUTO_REPLY_REGEX.read().unwrap();
    MULTIPLAYER_AUTO_REPLY_REGEX.read().unwrap();
    STEAM_SCAM.read().unwrap();
    STEAM_SCAM_IGNORE.read().unwrap();
}