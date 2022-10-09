use sparkle_cache::model::CachedEmoji;
use twilight_model::id::Id;

#[derive(Clone, Debug)]
pub struct QueriedEmoji {
    pub guild_id: i64,
    pub animated: bool,
    pub available: bool,
    pub id: i64,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
    pub user: Option<i64>,
}

impl From<QueriedEmoji> for CachedEmoji {
    fn from(emoji: QueriedEmoji) -> Self {
        Self {
            guild_id: Id::new(emoji.guild_id as u64),
            animated: emoji.animated,
            available: emoji.available,
            id: Id::new(emoji.id as u64),
            managed: emoji.managed,
            name: emoji.name,
            require_colons: emoji.require_colons,
            user: emoji.user.map(|id| Id::new(id as u64)),
        }
    }
}
