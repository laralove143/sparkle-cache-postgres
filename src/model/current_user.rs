use twilight_model::{
    id::Id,
    user::{CurrentUser, UserFlags},
    util::ImageHash,
};

use crate::model::premium_type_from_i16;

#[derive(Clone, Debug)]
pub struct QueriedCurrentUser {
    pub accent_color: Option<i64>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bot: bool,
    pub discriminator: i32,
    pub email: Option<String>,
    pub flags: Option<i64>,
    pub id: i64,
    pub locale: Option<String>,
    pub mfa_enabled: bool,
    pub name: String,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub verified: Option<bool>,
}

impl From<QueriedCurrentUser> for CurrentUser {
    fn from(user: QueriedCurrentUser) -> Self {
        Self {
            accent_color: user.accent_color.map(|color| color as u32),
            avatar: user
                .avatar
                .map(|avatar| ImageHash::parse(avatar.as_bytes()).unwrap()),
            banner: user
                .banner
                .map(|banner| ImageHash::parse(banner.as_bytes()).unwrap()),
            bot: user.bot,
            discriminator: user.discriminator as u16,
            email: user.email,
            flags: user
                .flags
                .map(|flags| UserFlags::from_bits_truncate(flags as u64)),
            id: Id::new(user.id as u64),
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type.map(premium_type_from_i16),
            public_flags: user
                .public_flags
                .map(|flags| UserFlags::from_bits_truncate(flags as u64)),
            verified: user.verified,
        }
    }
}
