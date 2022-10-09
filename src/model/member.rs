use sparkle_cache::model::CachedMember;
use twilight_model::{
    id::Id,
    user::UserFlags,
    util::{ImageHash, Timestamp},
};

use crate::model::premium_type_from_i16;

#[derive(Clone, Debug)]
pub struct QueriedMember {
    pub guild_avatar: Option<String>,
    pub communication_disabled_until: Option<i64>,
    pub deaf: bool,
    pub guild_id: i64,
    pub joined_at: i64,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<i64>,
    pub accent_color: Option<i64>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bot: bool,
    pub discriminator: i16,
    pub flags: Option<i64>,
    pub id: i64,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub system: Option<bool>,
}

impl From<QueriedMember> for CachedMember {
    fn from(member: QueriedMember) -> Self {
        Self {
            guild_avatar: member
                .guild_avatar
                .map(|avatar| ImageHash::parse(avatar.as_bytes()).unwrap()),
            communication_disabled_until: member
                .communication_disabled_until
                .map(|until| Timestamp::from_secs(until).unwrap()),
            deaf: member.deaf,
            guild_id: Id::new(member.guild_id as u64),
            joined_at: Timestamp::from_secs(member.joined_at).unwrap(),
            mute: member.mute,
            nick: member.nick,
            pending: member.pending,
            premium_since: member
                .premium_since
                .map(|since| Timestamp::from_secs(since).unwrap()),
            accent_color: member.accent_color.map(|color| color as u32),
            avatar: member
                .avatar
                .map(|avatar| ImageHash::parse(avatar.as_bytes()).unwrap()),
            banner: member
                .banner
                .map(|banner| ImageHash::parse(banner.as_bytes()).unwrap()),
            bot: member.bot,
            discriminator: member.discriminator as u16,
            flags: member
                .flags
                .map(|flags| UserFlags::from_bits_truncate(flags as u64)),
            id: Id::new(member.id as u64),
            locale: member.locale,
            mfa_enabled: member.mfa_enabled,
            name: member.name,
            premium_type: member.premium_type.map(premium_type_from_i16),
            public_flags: member
                .public_flags
                .map(|flags| UserFlags::from_bits_truncate(flags as u64)),
            system: member.system,
        }
    }
}
