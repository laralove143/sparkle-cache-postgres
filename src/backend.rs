#![allow(
    clippy::panic,
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::integer_arithmetic,
    clippy::std_instead_of_core,
    clippy::arithmetic_side_effects
)]

use std::borrow::Cow;

use async_trait::async_trait;
use sparkle_cache::{
    model::{
        CachedActivity, CachedAttachment, CachedChannel, CachedEmbed, CachedEmbedField,
        CachedEmoji, CachedGuild, CachedMember, CachedMessage, CachedPermissionOverwrite,
        CachedPresence, CachedReaction, CachedRole, CachedSticker,
    },
    Backend,
};
use sqlx::{query_file, query_file_as};
use twilight_model::{
    channel::StageInstance,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GenericMarker, GuildMarker, MessageMarker, RoleMarker,
            StageMarker, UserMarker,
        },
        Id,
    },
    user::CurrentUser,
    util::Timestamp,
};

use crate::{
    model::{
        embed::{QueriedEmbed, QueriedEmbedField},
        status_as_i16,
    },
    Cache,
};

#[async_trait]
impl Backend for Cache {
    type Error = sqlx::Error;

    async fn set_current_user(&self, current_user: CurrentUser) -> Result<(), Self::Error> {
        query_file!(
            "sql/set_current_user.sql",
            current_user.accent_color.map(|color| i64::from(color)),
            current_user.avatar.map(|avatar| avatar.to_string()),
            current_user.banner.map(|banner| banner.to_string()),
            current_user.bot,
            i32::from(current_user.discriminator),
            current_user.email,
            current_user.flags.map(|flags| flags.bits() as i64),
            current_user.id.get() as i64,
            current_user.locale,
            current_user.mfa_enabled,
            current_user.name,
            current_user
                .premium_type
                .map(|premium_type| premium_type as i16),
            current_user.public_flags.map(|flags| flags.bits() as i64),
            current_user.verified,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn upsert_channel(&self, channel: CachedChannel) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_channel.sql",
            channel.application_id.map(|id| id.get() as i64),
            channel.bitrate.map(|bitrate| i64::from(bitrate)),
            channel
                .default_auto_archive_duration
                .map(|duration| i32::from(duration.number())),
            channel.guild_id.map(|id| id.get() as i64),
            channel.icon.map(|icon| icon.to_string()),
            channel.id.get() as i64,
            channel.invitable,
            i16::from(u8::from(channel.kind)),
            channel.name,
            channel.nsfw,
            channel.owner_id.map(|id| id.get() as i64),
            channel.parent_id.map(|id| id.get() as i64),
            channel.position,
            channel.rate_limit_per_user.map(|limit| i32::from(limit)),
            channel.rtc_region,
            channel.thread_archived,
            channel
                .thread_auto_archive_duration
                .map(|duration| i32::from(duration.number())),
            channel.thread_archive_timestamp.map(Timestamp::as_secs),
            channel.thread_create_timestamp.map(Timestamp::as_secs),
            channel.thread_invitable,
            channel.thread_locked,
            channel.topic,
            channel.user_limit.map(|limit| i64::from(limit)),
            channel
                .video_quality_mode
                .map(|mode| i16::from(u8::from(mode))),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_channel.sql", channel_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_guild_channels(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_channels.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_permission_overwrite(
        &self,
        permission_overwrite: CachedPermissionOverwrite,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_permission_overwrite.sql",
            permission_overwrite.channel_id.get() as i64,
            permission_overwrite.allow.bits() as i64,
            permission_overwrite.deny.bits() as i64,
            permission_overwrite.id.get() as i64,
            i16::from(u8::from(permission_overwrite.kind)),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_channel_permission_overwrites(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_channel_permission_overwrites.sql",
            channel_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn upsert_message(&self, message: CachedMessage) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_message.sql",
            message.activity_type.map(|kind| i16::from(u8::from(kind))),
            message.activity_party_id,
            message
                .application_cover_image
                .map(|cover| cover.to_string()),
            message.application_description,
            message.application_icon.map(|avatar| avatar.to_string()),
            message.application_id.map(|id| id.get() as i64),
            message.application_name,
            message.interaction_application_id.map(|id| id.get() as i64),
            message.author.get() as i64,
            message.channel_id.get() as i64,
            message.content,
            message.edited_timestamp.map(Timestamp::as_secs),
            message.flags.map(|flags| flags.bits() as i64),
            message.guild_id.map(|id| id.get() as i64),
            message.id.get() as i64,
            i16::from(u8::from(message.kind)),
            message.mention_everyone,
            message.pinned,
            message.reference_channel_id.map(|id| id.get() as i64),
            message.reference_guild_id.map(|id| id.get() as i64),
            message.reference_message_id.map(|id| id.get() as i64),
            message.reference_fail_if_not_exists,
            message.referenced_message.map(|id| id.get() as i64),
            message.timestamp.as_secs(),
            message.thread.map(|id| id.get() as i64),
            message.tts,
            message.webhook_id.map(|id| id.get() as i64),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_message(&self, message_id: Id<MessageMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_message.sql", message_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_embed(&self, embed: CachedEmbed) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_embed.sql",
            embed.id.get() as i64,
            embed.message_id.get() as i64,
            embed.author_icon_url,
            embed.author_name,
            embed.author_proxy_icon_url,
            embed.author_url,
            embed.color.map(|color| i64::from(color)),
            embed.description,
            embed.footer_icon_url,
            embed.footer_proxy_icon_url,
            embed.footer_text,
            embed.image_height.map(|height| height as i64),
            embed.image_proxy_url,
            embed.image_url,
            embed.image_width.map(|width| width as i64),
            embed.kind,
            embed.provider_name,
            embed.provider_url,
            embed.thumbnail_height.map(|width| width as i64),
            embed.thumbnail_proxy_url,
            embed.thumbnail_url,
            embed.thumbnail_width.map(|width| width as i64),
            embed.timestamp.map(Timestamp::as_secs),
            embed.title,
            embed.url,
            embed.video_height.map(|width| width as i64),
            embed.video_proxy_url,
            embed.video_url,
            embed.video_width.map(|width| width as i64),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_embed(&self, embed_id: Id<GenericMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_embed.sql", embed_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_embed_field(&self, embed_field: CachedEmbedField) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_embed_field.sql",
            embed_field.embed_id.get() as i64,
            embed_field.inline,
            embed_field.name,
            embed_field.value,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_embed_fields(&self, embed_id: Id<GenericMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_embed_fields.sql", embed_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn select_message_embeds(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedEmbed>, Self::Error> {
        query_file_as!(
            QueriedEmbed,
            "sql/select_message_embeds.sql",
            message_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|embeds| embeds.into_iter().map(CachedEmbed::from).collect())
    }

    async fn select_embed_fields(
        &self,
        embed_id: Id<GenericMarker>,
    ) -> Result<Vec<CachedEmbedField>, Self::Error> {
        query_file_as!(
            QueriedEmbedField,
            "sql/select_embed_fields.sql",
            embed_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|fields| fields.into_iter().map(Into::into).collect())
    }

    async fn upsert_attachment(&self, attachment: CachedAttachment) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_attachment.sql",
            attachment.message_id.get() as i64,
            attachment.content_type,
            attachment.ephemeral,
            attachment.filename,
            attachment.description,
            attachment.height.map(|height| height as i64),
            attachment.id.get() as i64,
            attachment.proxy_url,
            attachment.size as i64,
            attachment.url,
            attachment.width.map(|width| width as i64),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_message_attachments(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_message_attachments.sql",
            message_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn upsert_reaction(&self, reaction: CachedReaction) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_reaction.sql",
            reaction.channel_id.get() as i64,
            reaction.emoji,
            reaction.guild_id.map(|id| id.get() as i64),
            reaction.message_id.get() as i64,
            reaction.user_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_reaction(
        &self,
        message_id: Id<MessageMarker>,
        user_id: Id<UserMarker>,
        emoji: String,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_reaction.sql",
            message_id.get() as i64,
            user_id.get() as i64,
            emoji
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_message_reactions_by_emoji(
        &self,
        message_id: Id<MessageMarker>,
        emoji: String,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_message_reactions_by_emoji.sql",
            message_id.get() as i64,
            emoji
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_message_reactions(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error> {
        query_file!("sql/delete_message_reactions.sql", message_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_member(&self, member: CachedMember) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_member.sql",
            member.guild_avatar.map(|avatar| avatar.to_string()),
            member.communication_disabled_until.map(Timestamp::as_secs),
            member.deaf,
            member.guild_id.get() as i64,
            member.joined_at.as_secs(),
            member.mute,
            member.nick,
            member.pending,
            member.premium_since.map(Timestamp::as_secs),
            member.accent_color.map(|color| i64::from(color)),
            member.avatar.map(|avatar| avatar.to_string()),
            member.banner.map(|banner| banner.to_string()),
            member.bot,
            member.discriminator as i16,
            member.flags.map(|flags| flags.bits() as i64),
            member.id.get() as i64,
            member.locale,
            member.mfa_enabled,
            member.name,
            member.premium_type.map(|kind| i16::from(kind as u8)),
            member.public_flags.map(|flags| flags.bits() as i64),
            member.system,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_member(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_member.sql",
            user_id.get() as i64,
            guild_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_guild_members(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_members.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_presence(&self, presence: CachedPresence) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_presence.sql",
            presence.guild_id.get() as i64,
            status_as_i16(presence.status),
            presence.user.get() as i64,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_presence(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_presence.sql",
            guild_id.get() as i64,
            user_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_guild_presences(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_presences.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_activity(&self, activity: CachedActivity) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_activity.sql",
            activity.user_id.get() as i64,
            activity.guild_id.get() as i64,
            activity.application_id.map(|id| id.get() as i64),
            activity.asset_large_image,
            activity.asset_large_text,
            activity.asset_small_image,
            activity.asset_small_text,
            activity.created_at.map(|timestamp| timestamp as i64),
            activity.details,
            activity.emoji_animated,
            activity.emoji_name,
            activity.emoji_id,
            activity.flags.map(|flags| flags.bits() as i64),
            activity.id,
            activity.instance,
            i16::from(u8::from(activity.kind)),
            activity.name,
            activity.party_id,
            activity.party_size_current.map(|size| size as i64),
            activity.party_size_max.map(|size| size as i64),
            activity.state,
            activity.timestamp_end.map(|timestamp| timestamp as i64),
            activity.timestamp_start.map(|timestamp| timestamp as i64),
            activity.url
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_user_activities(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_user_activities.sql",
            guild_id.get() as i64,
            user_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn upsert_guild(&self, guild: CachedGuild) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_guild.sql",
            guild.afk_channel_id.map(|id| id.get() as i64),
            guild.afk_timeout as i64,
            guild.application_id.map(|id| id.get() as i64),
            guild.banner.map(|banner| banner.to_string()),
            i16::from(u8::from(guild.default_message_notifications)),
            guild.description,
            guild.discovery_splash.map(|splash| splash.to_string()),
            i16::from(u8::from(guild.explicit_content_filter)),
            guild
                .features
                .iter()
                .map(|feature| Cow::from(feature.clone()))
                .collect::<Vec<_>>()
                .join(" "),
            guild.icon.map(|icon| icon.to_string()),
            guild.id.get() as i64,
            guild.joined_at.map(Timestamp::as_secs),
            guild.large,
            guild.max_members.map(|max| max as i64),
            guild.max_presences.map(|max| max as i64),
            guild.max_video_channel_users.map(|max| max as i64),
            i16::from(u8::from(guild.mfa_level)),
            guild.name,
            i16::from(u8::from(guild.nsfw_level)),
            guild.owner_id.get() as i64,
            guild.owner,
            guild
                .permissions
                .map(|permissions| permissions.bits() as i64),
            guild.preferred_locale,
            guild.premium_progress_bar_enabled,
            guild.premium_subscription_count.map(|count| count as i64),
            i16::from(u8::from(guild.premium_tier)),
            guild.rules_channel_id.map(|id| id.get() as i64),
            guild.splash.map(|splash| splash.to_string()),
            guild.system_channel_flags.bits() as i64,
            guild.system_channel_id.map(|id| id.get() as i64),
            guild.unavailable,
            guild.vanity_url_code,
            i16::from(u8::from(guild.verification_level)),
            guild.widget_channel_id.map(|id| id.get() as i64),
            guild.widget_enabled,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_guild(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn insert_role(&self, role: CachedRole) -> Result<(), Self::Error> {
        query_file!(
            "sql/insert_role.sql",
            role.guild_id.get() as i64,
            role.user_id.map(|id| id.get() as i64),
            i64::from(role.color),
            role.hoist,
            role.icon.map(|icon| icon.to_string()),
            role.id.get() as i64,
            role.managed,
            role.mentionable,
            role.name,
            role.permissions.bits() as i64,
            role.position,
            role.tags_bot_id.map(|id| id.get() as i64),
            role.tags_integration_id.map(|id| id.get() as i64),
            role.tags_premium_subscriber,
            role.unicode_emoji,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn update_roles(&self, role: CachedRole) -> Result<(), Self::Error> {
        query_file!(
            "sql/update_role.sql",
            role.guild_id.get() as i64,
            i64::from(role.color),
            role.hoist,
            role.icon.map(|icon| icon.to_string()),
            role.id.get() as i64,
            role.managed,
            role.mentionable,
            role.name,
            role.permissions.bits() as i64,
            role.position,
            role.tags_bot_id.map(|id| id.get() as i64),
            role.tags_integration_id.map(|id| id.get() as i64),
            role.tags_premium_subscriber,
            role.unicode_emoji,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_role(&self, role_id: Id<RoleMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_role.sql", role_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_guild_roles(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_roles.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_member_roles(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_member_roles.sql",
            guild_id.get() as i64,
            user_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn upsert_emoji(&self, emoji: CachedEmoji) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_emoji.sql",
            emoji.guild_id.get() as i64,
            emoji.animated,
            emoji.available,
            emoji.id.get() as i64,
            emoji.managed,
            emoji.name,
            emoji.require_colons,
            emoji.user.map(|id| id.get() as i64),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_emoji(&self, emoji_id: Id<EmojiMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_emoji.sql", emoji_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_guild_emojis(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_emojis.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_sticker(&self, sticker: CachedSticker) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_sticker.sql",
            sticker.message_id.map(|id| id.get() as i64),
            sticker.available,
            sticker.description,
            i16::from(u8::from(sticker.format_type)),
            sticker.guild_id.map(|id| id.get() as i64),
            sticker.id.get() as i64,
            sticker.kind.map(|kind| i16::from(u8::from(kind))),
            sticker.name,
            sticker.pack_id.map(|id| id.get() as i64),
            sticker.sort_value.map(|sort| sort as i64),
            sticker.tags,
            sticker.user_id.map(|id| id.get() as i64)
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_message_stickers(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error> {
        query_file!("sql/delete_message_stickers.sql", message_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_guild_stickers(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_guild_stickers.sql", guild_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn upsert_stage_instance(&self, stage: StageInstance) -> Result<(), Self::Error> {
        query_file!(
            "sql/upsert_stage_instance.sql",
            stage.channel_id.get() as i64,
            stage.guild_id.get() as i64,
            stage.guild_scheduled_event_id.map(|id| id.get() as i64),
            stage.id.get() as i64,
            i16::from(stage.privacy_level as u8),
            stage.topic,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    async fn delete_stage_instance(&self, stage_id: Id<StageMarker>) -> Result<(), Self::Error> {
        query_file!("sql/delete_stage_instance.sql", stage_id.get() as i64)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    async fn delete_guild_stage_instances(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Self::Error> {
        query_file!(
            "sql/delete_guild_stage_instances.sql",
            guild_id.get() as i64
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
