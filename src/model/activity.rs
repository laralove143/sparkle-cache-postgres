use sparkle_cache::model::CachedActivity;
use twilight_model::{
    gateway::presence::{ActivityFlags, ActivityType},
    id::Id,
};

#[derive(Clone, Debug)]
pub struct QueriedActivity {
    pub user_id: i64,
    pub guild_id: i64,
    pub application_id: Option<i64>,
    pub asset_large_image: Option<String>,
    pub asset_large_text: Option<String>,
    pub asset_small_image: Option<String>,
    pub asset_small_text: Option<String>,
    pub created_at: Option<i64>,
    pub details: Option<String>,
    pub emoji_animated: Option<bool>,
    pub emoji_name: Option<String>,
    pub emoji_id: Option<String>,
    pub flags: Option<i64>,
    pub id: Option<String>,
    pub instance: Option<bool>,
    pub kind: i16,
    pub name: String,
    pub party_id: Option<String>,
    pub party_size_current: Option<i64>,
    pub party_size_max: Option<i64>,
    pub state: Option<String>,
    pub timestamp_end: Option<i64>,
    pub timestamp_start: Option<i64>,
    pub url: Option<String>,
}

impl From<QueriedActivity> for CachedActivity {
    fn from(activity: QueriedActivity) -> Self {
        Self {
            user_id: Id::new(activity.user_id as u64),
            guild_id: Id::new(activity.guild_id as u64),
            application_id: activity.application_id.map(|id| Id::new(id as u64)),
            asset_large_image: activity.asset_large_image,
            asset_large_text: activity.asset_large_text,
            asset_small_image: activity.asset_small_image,
            asset_small_text: activity.asset_small_text,
            created_at: activity.created_at.map(|created| created as u64),
            details: activity.details,
            emoji_animated: activity.emoji_animated,
            emoji_name: activity.emoji_name,
            emoji_id: activity.emoji_id,
            flags: activity
                .flags
                .map(|flags| ActivityFlags::from_bits_truncate(flags as u64)),
            id: activity.id,
            instance: activity.instance,
            kind: ActivityType::from(activity.kind as u8),
            name: activity.name,
            party_id: activity.party_id,
            party_size_current: activity.party_size_current.map(|size| size as u64),
            party_size_max: activity.party_size_max.map(|size| size as u64),
            state: activity.state,
            timestamp_end: activity.timestamp_end.map(|end| end as u64),
            timestamp_start: activity.timestamp_start.map(|start| start as u64),
            url: activity.url,
        }
    }
}
