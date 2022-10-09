use sparkle_cache::model::CachedGuild;
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel, Permissions,
        PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::Id,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug)]
pub struct QueriedGuild {
    pub afk_channel_id: Option<i64>,
    pub afk_timeout: i64,
    pub application_id: Option<i64>,
    pub banner: Option<String>,
    pub default_message_notifications: i16,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub explicit_content_filter: i16,
    pub features: String,
    pub icon: Option<String>,
    pub id: i64,
    pub joined_at: Option<i64>,
    pub large: bool,
    pub max_members: Option<i64>,
    pub max_presences: Option<i64>,
    pub max_video_channel_users: Option<i64>,
    pub mfa_level: i16,
    pub name: String,
    pub nsfw_level: i16,
    pub owner_id: i64,
    pub owner: Option<bool>,
    pub permissions: Option<i64>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<i64>,
    pub premium_tier: i16,
    pub rules_channel_id: Option<i64>,
    pub splash: Option<String>,
    pub system_channel_flags: i64,
    pub system_channel_id: Option<i64>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: i16,
    pub widget_channel_id: Option<i64>,
    pub widget_enabled: Option<bool>,
}

impl From<QueriedGuild> for CachedGuild {
    fn from(guild: QueriedGuild) -> Self {
        Self {
            afk_channel_id: guild.afk_channel_id.map(|id| Id::new(id as u64)),
            afk_timeout: guild.afk_timeout as u64,
            application_id: guild.application_id.map(|id| Id::new(id as u64)),
            banner: guild
                .banner
                .map(|banner| ImageHash::parse(banner.as_bytes()).unwrap()),
            default_message_notifications: DefaultMessageNotificationLevel::from(
                guild.default_message_notifications as u8,
            ),
            description: guild.description,
            discovery_splash: guild
                .discovery_splash
                .map(|splash| ImageHash::parse(splash.as_bytes()).unwrap()),
            explicit_content_filter: ExplicitContentFilter::from(
                guild.explicit_content_filter as u8,
            ),
            features: guild
                .features
                .split(' ')
                .map(|feature| feature.to_owned().into())
                .collect(),
            icon: guild
                .icon
                .map(|icon| ImageHash::parse(icon.as_bytes()).unwrap()),
            id: Id::new(guild.id as u64),
            joined_at: guild
                .joined_at
                .map(|joined| Timestamp::from_secs(joined).unwrap()),
            large: guild.large,
            max_members: guild.max_members.map(|max| max as u64),
            max_presences: guild.max_presences.map(|max| max as u64),
            max_video_channel_users: guild.max_video_channel_users.map(|max| max as u64),
            mfa_level: MfaLevel::from(guild.mfa_level as u8),
            name: guild.name,
            nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
            owner_id: Id::new(guild.owner_id as u64),
            owner: guild.owner,
            permissions: guild
                .permissions
                .map(|flags| Permissions::from_bits_truncate(flags as u64)),
            preferred_locale: guild.preferred_locale,
            premium_progress_bar_enabled: guild.premium_progress_bar_enabled,
            premium_subscription_count: guild.premium_subscription_count.map(|count| count as u64),
            premium_tier: PremiumTier::from(guild.premium_tier as u8),
            rules_channel_id: guild.rules_channel_id.map(|id| Id::new(id as u64)),
            splash: guild
                .splash
                .map(|splash| ImageHash::parse(splash.as_bytes()).unwrap()),
            system_channel_flags: SystemChannelFlags::from_bits_truncate(
                guild.system_channel_flags as u64,
            ),
            system_channel_id: guild.system_channel_id.map(|id| Id::new(id as u64)),
            unavailable: guild.unavailable,
            vanity_url_code: guild.vanity_url_code,
            verification_level: VerificationLevel::from(guild.verification_level as u8),
            widget_channel_id: guild.widget_channel_id.map(|id| Id::new(id as u64)),
            widget_enabled: guild.widget_enabled,
        }
    }
}
