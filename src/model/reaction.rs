use sparkle_cache::model::CachedReaction;
use twilight_model::id::Id;

#[derive(Clone, Debug)]
pub struct QueriedReaction {
    pub channel_id: i64,
    pub emoji: String,
    pub guild_id: Option<i64>,
    pub message_id: i64,
    pub user_id: i64,
}

impl From<QueriedReaction> for CachedReaction {
    fn from(reaction: QueriedReaction) -> Self {
        Self {
            channel_id: Id::new(reaction.channel_id as u64),
            emoji: reaction.emoji,
            guild_id: reaction.guild_id.map(|id| Id::new(id as u64)),
            message_id: Id::new(reaction.message_id as u64),
            user_id: Id::new(reaction.user_id as u64),
        }
    }
}
