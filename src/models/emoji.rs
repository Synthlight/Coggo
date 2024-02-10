use std::num::NonZeroU64;

use serenity::all::{Context, Emoji, EmojiId, GuildId};

use crate::models::consts::*;

pub struct CachedEmoji {
    thumbs_up_testing: Option<Emoji>,
    thumbs_up_cog_hand: Option<Emoji>,
    thumbs_up_kappa: Option<Emoji>,
    thumbs_down_testing: Option<Emoji>,
    thumbs_down_cog_hand: Option<Emoji>,
    thumbs_down_shotgun: Option<Emoji>,
}

impl CachedEmoji {
    pub fn new() -> Self {
        return CachedEmoji {
            thumbs_up_testing: None,
            thumbs_up_cog_hand: None,
            thumbs_up_kappa: None,
            thumbs_down_testing: None,
            thumbs_down_cog_hand: None,
            thumbs_down_shotgun: None,
        };
    }

    pub async fn setup(&mut self, ctx: &Context) {
        if self.thumbs_up_testing.is_none() {
            println!("Fetching/caching emojis.");

            self.thumbs_up_testing = Some(GuildId::from(COGGO_TESTING).emoji(ctx, EmojiId::from(THUMBS_UP_ID_TESTING)).await.unwrap());
            self.thumbs_up_cog_hand = Some(GuildId::from(VOLCANOIDS).emoji(ctx, EmojiId::from(THUMBS_UP_ID_COG_HAND)).await.unwrap());
            self.thumbs_up_kappa = Some(GuildId::from(CAPS_SUB).emoji(ctx, EmojiId::from(THUMBS_UP_ID_KAPPA)).await.unwrap());
            self.thumbs_down_testing = Some(GuildId::from(COGGO_TESTING).emoji(ctx, EmojiId::from(THUMBS_DOWN_ID_TESTING)).await.unwrap());
            self.thumbs_down_cog_hand = Some(GuildId::from(VOLCANOIDS).emoji(ctx, EmojiId::from(THUMBS_DOWN_ID_COG_HAND)).await.unwrap());
            self.thumbs_down_shotgun = Some(GuildId::from(CAPS_SUB).emoji(ctx, EmojiId::from(THUMBS_DOWN_ID_SHOTGUN)).await.unwrap());

            println!("Emojis cached.");
        }
    }

    pub fn get_thumbs_up(&self, guild_id: NonZeroU64) -> Emoji {
        if guild_id == COGGO_TESTING {
            return self.thumbs_up_testing.clone().unwrap();
        } else if guild_id == CAPS_SUB {
            return self.thumbs_up_kappa.clone().unwrap();
        } else if guild_id == VOLCANOIDS {
            return self.thumbs_up_cog_hand.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }

    pub fn get_thumbs_down(&self, guild_id: NonZeroU64) -> Emoji {
        if guild_id == COGGO_TESTING {
            return self.thumbs_down_testing.clone().unwrap();
        } else if guild_id == CAPS_SUB {
            return self.thumbs_down_shotgun.clone().unwrap();
        } else if guild_id == VOLCANOIDS {
            return self.thumbs_down_cog_hand.clone().unwrap();
        } else {
            panic!("No DEBUG emoji found.");
        }
    }
}