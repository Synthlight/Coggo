struct CachedEmoji {
    thumbs_up_testing: Option<Emoji>,
    thumbs_up_cog_hand: Option<Emoji>,
    thumbs_up_kappa: Option<Emoji>,
    thumbs_down_testing: Option<Emoji>,
    thumbs_down_cog_hand: Option<Emoji>,
    thumbs_down_shotgun: Option<Emoji>,
}

impl CachedEmoji {
    fn new() -> Self {
        return CachedEmoji {
            thumbs_up_testing: None,
            thumbs_up_cog_hand: None,
            thumbs_up_kappa: None,
            thumbs_down_testing: None,
            thumbs_down_cog_hand: None,
            thumbs_down_shotgun: None,
        };
    }

    async fn setup_emoji(&mut self, ctx: &Context) {
        if self.thumbs_up_testing.is_none() {
            println!("Fetching/caching emojis.");

            self.thumbs_up_testing = Some(GuildId(COGGO_TESTING).emoji(ctx, EmojiId(THUMBS_UP_ID_TESTING)).await.unwrap());
            self.thumbs_up_cog_hand = Some(GuildId(VOLCANOIDS).emoji(ctx, EmojiId(THUMBS_UP_ID_COG_HAND)).await.unwrap());
            self.thumbs_up_kappa = Some(GuildId(CAPS_SUB).emoji(ctx, EmojiId(THUMBS_UP_ID_KAPPA)).await.unwrap());
            self.thumbs_down_testing = Some(GuildId(COGGO_TESTING).emoji(ctx, EmojiId(THUMBS_DOWN_ID_TESTING)).await.unwrap());
            self.thumbs_down_cog_hand = Some(GuildId(VOLCANOIDS).emoji(ctx, EmojiId(THUMBS_DOWN_ID_COG_HAND)).await.unwrap());
            self.thumbs_down_shotgun = Some(GuildId(CAPS_SUB).emoji(ctx, EmojiId(THUMBS_DOWN_ID_SHOTGUN)).await.unwrap());

            println!("Emojis cached.");
        }
    }

    fn get_thumbs_up(&self, guild_id: &u64) -> Emoji {
        if guild_id == &COGGO_TESTING {
            return self.thumbs_up_testing.clone().unwrap();
        } else if guild_id == &CAPS_SUB {
            return self.thumbs_up_kappa.clone().unwrap();
        } else if guild_id == &VOLCANOIDS {
            return self.thumbs_up_cog_hand.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }

    fn get_thumbs_down(&self, guild_id: &u64) -> Emoji {
        if guild_id == &COGGO_TESTING {
            return self.thumbs_down_testing.clone().unwrap();
        } else if guild_id == &CAPS_SUB {
            return self.thumbs_down_shotgun.clone().unwrap();
        } else if guild_id == &VOLCANOIDS {
            return self.thumbs_down_cog_hand.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }
}