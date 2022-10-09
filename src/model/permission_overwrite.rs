use sparkle_cache::model::CachedPermissionOverwrite;
use twilight_model::{
    channel::permission_overwrite::PermissionOverwriteType, guild::Permissions, id::Id,
};

#[derive(Clone, Copy, Debug)]
pub struct QueriedPermissionOverwrite {
    pub channel_id: i64,
    pub allow: i64,
    pub deny: i64,
    pub id: i64,
    pub kind: i16,
}

impl From<QueriedPermissionOverwrite> for CachedPermissionOverwrite {
    fn from(overwrite: QueriedPermissionOverwrite) -> Self {
        Self {
            channel_id: Id::new(overwrite.channel_id as u64),
            allow: Permissions::from_bits_truncate(overwrite.allow as u64),
            deny: Permissions::from_bits_truncate(overwrite.deny as u64),
            id: Id::new(overwrite.id as u64),
            kind: PermissionOverwriteType::from(overwrite.kind as u8),
        }
    }
}
