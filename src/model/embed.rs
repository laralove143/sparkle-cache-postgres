#![allow(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items
)]

use sparkle_cache::model::{CachedEmbed, CachedEmbedField};
use twilight_model::{id::Id, util::Timestamp};

/// Embed returned from SQL select queries
#[derive(Clone, Debug)]
pub struct QueriedEmbed {
    pub id: i64,
    pub message_id: i64,
    pub author_icon_url: Option<String>,
    pub author_name: Option<String>,
    pub author_proxy_icon_url: Option<String>,
    pub author_url: Option<String>,
    pub color: Option<i64>,
    pub description: Option<String>,
    pub footer_icon_url: Option<String>,
    pub footer_proxy_icon_url: Option<String>,
    pub footer_text: Option<String>,
    pub image_height: Option<i64>,
    pub image_proxy_url: Option<String>,
    pub image_url: Option<String>,
    pub image_width: Option<i64>,
    pub kind: String,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub thumbnail_height: Option<i64>,
    pub thumbnail_proxy_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub timestamp: Option<i64>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video_height: Option<i64>,
    pub video_proxy_url: Option<String>,
    pub video_url: Option<String>,
    pub video_width: Option<i64>,
}

impl From<QueriedEmbed> for CachedEmbed {
    fn from(embed: QueriedEmbed) -> Self {
        Self {
            id: Id::new(embed.id as u64),
            message_id: Id::new(embed.message_id as u64),
            author_icon_url: embed.author_icon_url,
            author_name: embed.author_name,
            author_proxy_icon_url: embed.author_proxy_icon_url,
            author_url: embed.author_url,
            color: embed.color.map(|color| color as u32),
            description: embed.description,
            footer_icon_url: embed.footer_icon_url,
            footer_proxy_icon_url: embed.footer_proxy_icon_url,
            footer_text: embed.footer_text,
            image_height: embed.image_height.map(|height| height as u64),
            image_proxy_url: embed.image_proxy_url,
            image_url: embed.image_url,
            image_width: embed.image_width.map(|width| width as u64),
            kind: embed.kind,
            provider_name: embed.provider_name,
            provider_url: embed.provider_url,
            thumbnail_height: embed.thumbnail_height.map(|height| height as u64),
            thumbnail_proxy_url: embed.thumbnail_proxy_url,
            thumbnail_url: embed.thumbnail_url,
            thumbnail_width: embed.thumbnail_width.map(|width| width as u64),
            timestamp: embed
                .timestamp
                .map(|timestamp| Timestamp::from_secs(timestamp).unwrap()),
            title: embed.title,
            url: embed.url,
            video_height: embed.video_height.map(|height| height as u64),
            video_proxy_url: embed.video_proxy_url,
            video_url: embed.video_url,
            video_width: embed.video_width.map(|width| width as u64),
        }
    }
}

/// Embed field returned from SQL select queries
#[derive(Clone, Debug)]
pub struct QueriedEmbedField {
    pub embed_id: i64,
    pub inline: bool,
    pub name: String,
    pub value: String,
}

impl From<QueriedEmbedField> for CachedEmbedField {
    fn from(field: QueriedEmbedField) -> Self {
        Self {
            embed_id: Id::new(field.embed_id as u64),
            inline: field.inline,
            name: field.name,
            value: field.value,
        }
    }
}
