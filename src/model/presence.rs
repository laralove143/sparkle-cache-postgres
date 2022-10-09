use sparkle_cache::model::CachedPresence;
use twilight_model::id::Id;

use crate::model::status_from_i16;

#[derive(Clone, Copy, Debug)]
pub struct QueriedPresence {
    pub guild_id: i64,
    pub status: i16,
    pub user: i64,
}

impl From<QueriedPresence> for CachedPresence {
    fn from(presence: QueriedPresence) -> Self {
        Self {
            guild_id: Id::new(presence.guild_id as u64),
            status: status_from_i16(presence.status),
            user: Id::new(presence.user as u64),
        }
    }
}
