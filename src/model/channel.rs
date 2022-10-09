use sparkle_cache::model::CachedChannel;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, ChannelType, VideoQualityMode},
    id::Id,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug)]
pub struct QueriedChannel {
    pub application_id: Option<i64>,
    pub bitrate: Option<i64>,
    pub default_auto_archive_duration: Option<i32>,
    pub guild_id: Option<i64>,
    pub icon: Option<String>,
    pub id: i64,
    pub invitable: Option<bool>,
    pub kind: i16,
    pub name: Option<String>,
    pub nsfw: Option<bool>,
    pub owner_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub rtc_region: Option<String>,
    pub thread_archived: Option<bool>,
    pub thread_auto_archive_duration: Option<i32>,
    pub thread_archive_timestamp: Option<i64>,
    pub thread_create_timestamp: Option<i64>,
    pub thread_invitable: Option<bool>,
    pub thread_locked: Option<bool>,
    pub topic: Option<String>,
    pub user_limit: Option<i64>,
    pub video_quality_mode: Option<i16>,
}

impl From<QueriedChannel> for CachedChannel {
    fn from(channel: QueriedChannel) -> Self {
        Self {
            application_id: channel.application_id.map(|id| Id::new(id as u64)),
            bitrate: channel.bitrate.map(|bitrate| bitrate as u32),
            default_auto_archive_duration: channel
                .default_auto_archive_duration
                .map(|duration| AutoArchiveDuration::from(duration as u16)),
            guild_id: channel.guild_id.map(|id| Id::new(id as u64)),
            icon: channel
                .icon
                .map(|icon| ImageHash::parse(icon.as_bytes()).unwrap()),
            id: Id::new(channel.id as u64),
            invitable: channel.invitable,
            kind: ChannelType::from(channel.kind as u8),
            name: channel.name,
            nsfw: channel.nsfw,
            owner_id: channel.owner_id.map(|id| Id::new(id as u64)),
            parent_id: channel.parent_id.map(|id| Id::new(id as u64)),
            position: channel.position,
            rate_limit_per_user: channel.rate_limit_per_user.map(|limit| limit as u16),
            rtc_region: channel.rtc_region,
            thread_archived: channel.thread_archived,
            thread_auto_archive_duration: channel
                .thread_auto_archive_duration
                .map(|duration| AutoArchiveDuration::from(duration as u16)),
            thread_archive_timestamp: channel
                .thread_archive_timestamp
                .map(|archive| Timestamp::from_secs(archive).unwrap()),
            thread_create_timestamp: channel
                .thread_create_timestamp
                .map(|create| Timestamp::from_secs(create).unwrap()),
            thread_invitable: channel.thread_invitable,
            thread_locked: channel.thread_locked,
            topic: channel.topic,
            user_limit: channel.user_limit.map(|limit| limit as u32),
            video_quality_mode: channel
                .video_quality_mode
                .map(|quality| VideoQualityMode::from(quality as u8)),
        }
    }
}
