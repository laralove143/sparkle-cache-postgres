use sparkle_cache::model::CachedMessage;
use twilight_model::{
    channel::message::{MessageActivityType, MessageFlags, MessageType},
    id::Id,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug)]
pub struct QueriedMessage {
    pub activity_type: Option<i16>,
    pub activity_party_id: Option<String>,
    pub application_cover_image: Option<String>,
    pub application_description: Option<String>,
    pub application_icon: Option<String>,
    pub application_id: Option<i64>,
    pub application_name: Option<String>,
    pub interaction_application_id: Option<i64>,
    pub author: i64,
    pub channel_id: i64,
    pub content: String,
    pub edited_timestamp: Option<i64>,
    pub flags: Option<i64>,
    pub guild_id: Option<i64>,
    pub id: i64,
    pub kind: i16,
    pub mention_everyone: bool,
    pub pinned: bool,
    pub reference_channel_id: Option<i64>,
    pub reference_guild_id: Option<i64>,
    pub reference_message_id: Option<i64>,
    pub reference_fail_if_not_exists: Option<bool>,
    pub referenced_message: Option<i64>,
    pub timestamp: i64,
    pub thread: Option<i64>,
    pub tts: bool,
    pub webhook_id: Option<i64>,
}

impl From<QueriedMessage> for CachedMessage {
    fn from(message: QueriedMessage) -> Self {
        Self {
            activity_type: message
                .activity_type
                .map(|activity| MessageActivityType::from(activity as u8)),
            activity_party_id: message.activity_party_id,
            application_cover_image: message
                .application_cover_image
                .map(|cover| ImageHash::parse(cover.as_bytes()).unwrap()),
            application_description: message.application_description,
            application_icon: message
                .application_icon
                .map(|icon| ImageHash::parse(icon.as_bytes()).unwrap()),
            application_id: message.application_id.map(|id| Id::new(id as u64)),
            application_name: message.application_name,
            interaction_application_id: message
                .interaction_application_id
                .map(|id| Id::new(id as u64)),
            author: Id::new(message.author as u64),
            channel_id: Id::new(message.channel_id as u64),
            content: message.content,
            edited_timestamp: message
                .edited_timestamp
                .map(|edited| Timestamp::from_secs(edited).unwrap()),
            flags: message
                .flags
                .map(|flags| MessageFlags::from_bits_truncate(flags as u64)),
            guild_id: message.guild_id.map(|id| Id::new(id as u64)),
            id: Id::new(message.id as u64),
            kind: MessageType::from(message.kind as u8),
            mention_everyone: message.mention_everyone,
            pinned: message.pinned,
            reference_channel_id: message.reference_channel_id.map(|id| Id::new(id as u64)),
            reference_guild_id: message.reference_guild_id.map(|id| Id::new(id as u64)),
            reference_message_id: message.reference_message_id.map(|id| Id::new(id as u64)),
            reference_fail_if_not_exists: message.reference_fail_if_not_exists,
            referenced_message: message.referenced_message.map(|id| Id::new(id as u64)),
            timestamp: Timestamp::from_secs(message.timestamp).unwrap(),
            thread: message.thread.map(|id| Id::new(id as u64)),
            tts: message.tts,
            webhook_id: message.webhook_id.map(|id| Id::new(id as u64)),
        }
    }
}
