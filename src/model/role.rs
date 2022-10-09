use sparkle_cache::model::CachedRole;
use twilight_model::{guild::Permissions, id::Id, util::ImageHash};

#[derive(Clone, Debug)]
pub struct QueriedRole {
    pub guild_id: i64,
    pub user_id: Option<i64>,
    pub color: i64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub id: i64,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: i64,
    pub position: i64,
    pub tags_bot_id: Option<i64>,
    pub tags_integration_id: Option<i64>,
    pub tags_premium_subscriber: Option<bool>,
    pub unicode_emoji: Option<String>,
}

impl From<QueriedRole> for CachedRole {
    fn from(role: QueriedRole) -> Self {
        Self {
            guild_id: Id::new(role.guild_id as u64),
            user_id: role.user_id.map(|id| Id::new(id as u64)),
            color: role.color as u32,
            hoist: role.hoist,
            icon: role
                .icon
                .map(|icon| ImageHash::parse(icon.as_bytes()).unwrap()),
            id: Id::new(role.id as u64),
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: Permissions::from_bits_truncate(role.permissions as u64),
            position: role.position,
            tags_bot_id: role.tags_bot_id.map(|id| Id::new(id as u64)),
            tags_integration_id: role.tags_integration_id.map(|id| Id::new(id as u64)),
            tags_premium_subscriber: role.tags_premium_subscriber,
            unicode_emoji: role.unicode_emoji,
        }
    }
}
