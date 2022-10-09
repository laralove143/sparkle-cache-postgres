use sparkle_cache::model::CachedAttachment;
use twilight_model::id::Id;

#[derive(Clone, Debug)]
pub struct QueriedAttachment {
    pub message_id: i64,
    pub content_type: Option<String>,
    pub ephemeral: bool,
    pub filename: String,
    pub description: Option<String>,
    pub height: Option<i64>,
    pub id: i64,
    pub proxy_url: String,
    pub size: i64,
    pub url: String,
    pub width: Option<i64>,
}

impl From<QueriedAttachment> for CachedAttachment {
    fn from(attachment: QueriedAttachment) -> Self {
        Self {
            message_id: Id::new(attachment.message_id as u64),
            content_type: attachment.content_type,
            ephemeral: attachment.ephemeral,
            filename: attachment.filename,
            description: attachment.description,
            height: attachment.height.map(|height| height as u64),
            id: Id::new(attachment.id as u64),
            proxy_url: attachment.proxy_url,
            size: attachment.size as u64,
            url: attachment.url,
            width: attachment.width.map(|width| width as u64),
        }
    }
}
