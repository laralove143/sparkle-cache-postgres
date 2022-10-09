use sparkle_cache::model::CachedSticker;
use twilight_model::{
    channel::message::sticker::{StickerFormatType, StickerType},
    id::Id,
};

#[derive(Clone, Debug)]
pub struct QueriedSticker {
    pub message_id: Option<i64>,
    pub available: Option<bool>,
    pub description: Option<String>,
    pub format_type: i16,
    pub guild_id: Option<i64>,
    pub id: i64,
    pub kind: Option<i16>,
    pub name: String,
    pub pack_id: Option<i64>,
    pub sort_value: Option<i64>,
    pub tags: Option<String>,
    pub user_id: Option<i64>,
}

impl From<QueriedSticker> for CachedSticker {
    fn from(sticker: QueriedSticker) -> Self {
        Self {
            message_id: sticker.message_id.map(|id| Id::new(id as u64)),
            available: sticker.available,
            description: sticker.description,
            format_type: StickerFormatType::from(sticker.format_type as u8),
            guild_id: sticker.guild_id.map(|id| Id::new(id as u64)),
            id: Id::new(sticker.id as u64),
            kind: sticker.kind.map(|kind| StickerType::from(kind as u8)),
            name: sticker.name,
            pack_id: sticker.pack_id.map(|id| Id::new(id as u64)),
            sort_value: sticker.sort_value.map(|sort| sort as u64),
            tags: sticker.tags,
            user_id: sticker.user_id.map(|id| Id::new(id as u64)),
        }
    }
}
