#![allow(
    clippy::std_instead_of_core,
    clippy::as_conversions,
    clippy::arithmetic,
    clippy::integer_arithmetic,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use async_trait::async_trait;
use sparkle_cache::{
    cache::Error,
    model::{
        CachedActivity, CachedAttachment, CachedChannel, CachedEmoji, CachedGuild, CachedMember,
        CachedMessage, CachedPermissionOverwrite, CachedPresence, CachedReaction, CachedRole,
        CachedSticker,
    },
    Cache as CacheTrait,
};
use sqlx::{query_file_as, query_file_scalar};
use twilight_model::{
    channel::StageInstance,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GuildMarker, MessageMarker, RoleMarker, StageMarker,
            StickerMarker, UserMarker,
        },
        Id,
    },
    user::CurrentUser,
};

use crate::{
    model::{
        activity::QueriedActivity, attachment::QueriedAttachment, channel::QueriedChannel,
        current_user::QueriedCurrentUser, emoji::QueriedEmoji, guild::QueriedGuild,
        member::QueriedMember, message::QueriedMessage,
        permission_overwrite::QueriedPermissionOverwrite, presence::QueriedPresence,
        reaction::QueriedReaction, role::QueriedRole, stage_instance::QueriedStageInstance,
        sticker::QueriedSticker,
    },
    Cache,
};

#[async_trait]
impl CacheTrait for Cache {
    async fn current_user(&self) -> Result<CurrentUser, Error<Self::Error>> {
        query_file_as!(QueriedCurrentUser, "sql/select_current_user.sql")
            .fetch_one(&self.0)
            .await
            .map(Into::into)
            .map_err(|_err| Error::CurrentUserMissing)
    }

    async fn channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Option<CachedChannel>, Error<Self::Error>> {
        query_file_as!(
            QueriedChannel,
            "sql/select_channel.sql",
            channel_id.get() as i64
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }

    async fn guild_channels(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedChannel>, Error<Self::Error>> {
        query_file_as!(
            QueriedChannel,
            "sql/select_channel.sql",
            guild_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|channels| channels.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn permission_overwrites(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Vec<CachedPermissionOverwrite>, Error<Self::Error>> {
        query_file_as!(
            QueriedPermissionOverwrite,
            "sql/select_permission_overwrites.sql",
            channel_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|overwrites| overwrites.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn private_channel(
        &self,
        recipient_id: Id<UserMarker>,
    ) -> Result<Option<Id<ChannelMarker>>, Error<Self::Error>> {
        query_file_scalar!("sql/select_private_channel.sql", recipient_id.get() as i64)
            .fetch_optional(&self.0)
            .await
            .map(|opt| opt.map(|id| Id::new(id as u64)))
            .map_err(Error::Backend)
    }

    async fn message(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Option<CachedMessage>, Error<Self::Error>> {
        query_file_as!(
            QueriedMessage,
            "sql/select_message.sql",
            message_id.get() as i64
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }

    async fn attachments(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedAttachment>, Error<Self::Error>> {
        query_file_as!(
            QueriedAttachment,
            "sql/select_attachments.sql",
            message_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|attachments| attachments.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn reactions(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedReaction>, Error<Self::Error>> {
        query_file_as!(
            QueriedReaction,
            "sql/select_reactions.sql",
            message_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|reactions| reactions.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn stickers(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedSticker>, Error<Self::Error>> {
        query_file_as!(
            QueriedSticker,
            "sql/select_stickers.sql",
            message_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|stickers| stickers.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<Option<CachedMember>, Error<Self::Error>> {
        query_file_as!(
            QueriedMember,
            "sql/select_member.sql",
            guild_id.get() as i64,
            user_id.get() as i64,
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }

    async fn member_roles(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedRole>, Error<Self::Error>> {
        query_file_as!(
            QueriedRole,
            "sql/select_member_roles.sql",
            user_id.get() as i64,
            guild_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|roles| roles.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn presence(
        &self,
        user_id: Id<UserMarker>,
    ) -> Result<Option<CachedPresence>, Error<Self::Error>> {
        query_file_as!(
            QueriedPresence,
            "sql/select_presence.sql",
            user_id.get() as i64
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }

    async fn member_activities(
        &self,
        user_id: Id<UserMarker>,
    ) -> Result<Vec<CachedActivity>, Error<Self::Error>> {
        query_file_as!(
            QueriedActivity,
            "sql/select_member_activities.sql",
            user_id.get() as i64
        )
        .fetch_all(&self.0)
        .await
        .map(|activities| activities.into_iter().map(Into::into).collect())
        .map_err(Error::Backend)
    }

    async fn guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Option<CachedGuild>, Error<Self::Error>> {
        query_file_as!(QueriedGuild, "sql/select_guild.sql", guild_id.get() as i64)
            .fetch_optional(&self.0)
            .await
            .map(|opt| opt.map(Into::into))
            .map_err(Error::Backend)
    }

    async fn role(
        &self,
        role_id: Id<RoleMarker>,
    ) -> Result<Option<CachedRole>, Error<Self::Error>> {
        query_file_as!(QueriedRole, "sql/select_role.sql", role_id.get() as i64)
            .fetch_optional(&self.0)
            .await
            .map(|opt| opt.map(Into::into))
            .map_err(Error::Backend)
    }

    async fn emoji(
        &self,
        emoji_id: Id<EmojiMarker>,
    ) -> Result<Option<CachedEmoji>, Error<Self::Error>> {
        query_file_as!(QueriedEmoji, "sql/select_emoji.sql", emoji_id.get() as i64)
            .fetch_optional(&self.0)
            .await
            .map(|opt| opt.map(Into::into))
            .map_err(Error::Backend)
    }

    async fn sticker(
        &self,
        sticker_id: Id<StickerMarker>,
    ) -> Result<Option<CachedSticker>, Error<Self::Error>> {
        query_file_as!(
            QueriedSticker,
            "sql/select_sticker.sql",
            sticker_id.get() as i64
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }

    async fn stage_instance(
        &self,
        stage_id: Id<StageMarker>,
    ) -> Result<Option<StageInstance>, Error<Self::Error>> {
        query_file_as!(
            QueriedStageInstance,
            "sql/select_stage_instance.sql",
            stage_id.get() as i64
        )
        .fetch_optional(&self.0)
        .await
        .map(|opt| opt.map(Into::into))
        .map_err(Error::Backend)
    }
}
