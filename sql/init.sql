CREATE TABLE IF NOT EXISTS "current_user"
(
    accent_color  bigint,
    avatar        char(16),
    banner        char(16),
    bot           bool    NOT NULL,
    discriminator integer NOT NULL,
    email         text,
    flags         bigint,
    id            bigint PRIMARY KEY,
    locale        text,
    mfa_enabled   bool    NOT NULL,
    name          text    NOT NULL,
    premium_type  smallint,
    public_flags  bigint,
    verified      bool
);

CREATE TABLE IF NOT EXISTS channels
(
    application_id                bigint,
    bitrate                       bigint,
    default_auto_archive_duration integer,
    guild_id                      bigint,
    icon                          char(16),
    id                            bigint PRIMARY KEY,
    invitable                     bool,
    kind                          smallint NOT NULL,
    name                          text,
    nsfw                          bool,
    owner_id                      bigint,
    parent_id                     bigint,
    position                      integer,
    rate_limit_per_user           integer,
    rtc_region                    text,
    thread_archived               bool,
    thread_auto_archive_duration  integer,
    thread_archive_timestamp      bigint,
    thread_create_timestamp       bigint,
    thread_invitable              bool,
    thread_locked                 bool,
    topic                         text,
    user_limit                    bigint,
    video_quality_mode            smallint
);
CREATE INDEX IF NOT EXISTS channels_guild_id_idx ON channels (guild_id);
CREATE INDEX IF NOT EXISTS channels_name_idx ON channels (name);

CREATE TABLE IF NOT EXISTS permission_overwrites
(
    channel_id bigint   NOT NULL,
    allow      bigint   NOT NULL,
    deny       bigint   NOT NULL,
    id         bigint   NOT NULL,
    kind       smallint NOT NULL
);
CREATE INDEX IF NOT EXISTS permission_overwrites_channel_id_idx ON permission_overwrites (channel_id);

CREATE TABLE IF NOT EXISTS private_channels
(
    channel_id bigint NOT NULL,
    user_id    bigint PRIMARY KEY
);
CREATE UNIQUE INDEX IF NOT EXISTS private_channels_idx ON private_channels (channel_id);

CREATE TABLE IF NOT EXISTS guilds
(
    afk_channel_id                bigint,
    afk_timeout                   bigint   NOT NULL,
    application_id                bigint,
    banner                        char(16),
    default_message_notifications smallint NOT NULL,
    description                   text,
    discovery_splash              char(16),
    explicit_content_filter       smallint NOT NULL,
    features                      text     NOT NULL,
    icon                          char(16),
    id                            bigint PRIMARY KEY,
    joined_at                     bigint,
    large                         bool     NOT NULL,
    max_members                   bigint,
    max_presences                 bigint,
    max_video_channel_users       bigint,
    mfa_level                     smallint NOT NULL,
    name                          text     NOT NULL,
    nsfw_level                    smallint NOT NULL,
    owner_id                      bigint   NOT NULL,
    owner                         bool,
    permissions                   bigint,
    preferred_locale              text     NOT NULL,
    premium_progress_bar_enabled  bool     NOT NULL,
    premium_subscription_count    bigint,
    premium_tier                  smallint NOT NULL,
    rules_channel_id              bigint,
    splash                        char(16),
    system_channel_flags          bigint   NOT NULL,
    system_channel_id             bigint,
    unavailable                   bool     NOT NULL,
    vanity_url_code               text,
    verification_level            smallint NOT NULL,
    widget_channel_id             bigint,
    widget_enabled                bool
);
CREATE INDEX IF NOT EXISTS guilds_name_idx ON guilds (name);

CREATE TABLE IF NOT EXISTS emojis
(
    guild_id       bigint NOT NULL,
    animated       bool   NOT NULL,
    available      bool   NOT NULL,
    id             bigint PRIMARY KEY,
    managed        bool   NOT NULL,
    name           text   NOT NULL,
    require_colons bool   NOT NULL,
    "user"         bigint
);
CREATE INDEX IF NOT EXISTS emojis_guild_id_idx ON emojis (guild_id);
CREATE INDEX IF NOT EXISTS emojis_name_idx ON emojis (name);

CREATE TABLE IF NOT EXISTS stickers
(
    message_id  bigint,
    available   bool,
    description text,
    format_type smallint NOT NULL,
    guild_id    bigint,
    id          bigint   NOT NULL,
    kind        smallint,
    name        text     NOT NULL,
    pack_id     bigint,
    sort_value  bigint,
    tags        text,
    user_id     bigint
);
CREATE INDEX IF NOT EXISTS stickers_idx ON stickers (id);
CREATE INDEX IF NOT EXISTS stickers_message_id_idx ON stickers (message_id);
CREATE INDEX IF NOT EXISTS stickers_guild_id_idx ON stickers (guild_id);
CREATE INDEX IF NOT EXISTS stickers_pack_id_idx ON stickers (pack_id);
CREATE INDEX IF NOT EXISTS stickers_name_idx ON stickers (name);

CREATE TABLE IF NOT EXISTS members
(
    guild_avatar                 char(16),
    communication_disabled_until bigint,
    deaf                         bool     NOT NULL,
    guild_id                     bigint   NOT NULL,
    joined_at                    bigint   NOT NULL,
    mute                         bool     NOT NULL,
    nick                         text,
    pending                      bool     NOT NULL,
    premium_since                bigint,
    accent_color                 bigint,
    avatar                       char(16),
    banner                       char(16),
    bot                          bool     NOT NULL,
    discriminator                smallint NOT NULL,
    flags                        bigint,
    id                           bigint   NOT NULL,
    locale                       text,
    mfa_enabled                  bool,
    name                         text     NOT NULL,
    premium_type                 smallint,
    public_flags                 bigint,
    system                       bool
);
CREATE UNIQUE INDEX IF NOT EXISTS members_idx ON members (guild_id, id);
CREATE INDEX IF NOT EXISTS members_guild_id_idx ON members (guild_id);
CREATE INDEX IF NOT EXISTS members_user_id_idx ON members (id);
CREATE INDEX IF NOT EXISTS members_nick_idx ON members (nick);
CREATE INDEX IF NOT EXISTS members_name_idx ON members (name);

CREATE TABLE IF NOT EXISTS messages
(
    activity_type                smallint,
    activity_party_id            text,
    application_cover_image      char(16),
    application_description      text,
    application_icon             char(16),
    application_id               bigint,
    application_name             text,
    interaction_application_id   bigint,
    author                       bigint   NOT NULL,
    channel_id                   bigint   NOT NULL,
    content                      text     NOT NULL,
    edited_timestamp             bigint,
    flags                        bigint,
    guild_id                     bigint,
    id                           bigint PRIMARY KEY,
    kind                         smallint NOT NULL,
    mention_everyone             bool     NOT NULL,
    pinned                       bool     NOT NULL,
    reference_channel_id         bigint,
    reference_guild_id           bigint,
    reference_message_id         bigint,
    reference_fail_if_not_exists bool,
    referenced_message           bigint,
    timestamp                    bigint   NOT NULL,
    thread                       bigint,
    tts                          bool     NOT NULL,
    webhook_id                   bigint
);
CREATE INDEX IF NOT EXISTS messages_author_idx ON messages (author);
CREATE INDEX IF NOT EXISTS messages_channel_id_idx ON messages (channel_id);
CREATE INDEX IF NOT EXISTS messages_guild_id_idx ON messages (guild_id);

CREATE TABLE IF NOT EXISTS embeds
(
    id                    bigint PRIMARY KEY,
    message_id            bigint NOT NULL,
    author_icon_url       text,
    author_name           text,
    author_proxy_icon_url text,
    author_url            text,
    color                 bigint,
    description           text,
    footer_icon_url       text,
    footer_proxy_icon_url text,
    footer_text           text,
    image_height          bigint,
    image_proxy_url       text,
    image_url             text,
    image_width           bigint,
    kind                  text   NOT NULL,
    provider_name         text,
    provider_url          text,
    thumbnail_height      bigint,
    thumbnail_proxy_url   text,
    thumbnail_url         text,
    thumbnail_width       bigint,
    timestamp             bigint,
    title                 text,
    url                   text,
    video_height          bigint,
    video_proxy_url       text,
    video_url             text,
    video_width           bigint
);
CREATE INDEX IF NOT EXISTS embeds_message_id_idx ON embeds (message_id);

CREATE TABLE IF NOT EXISTS embed_fields
(
    embed_id bigint NOT NULL,
    inline   bool   NOT NULL,
    name     text   NOT NULL,
    value    text   NOT NULL
);
CREATE INDEX IF NOT EXISTS embed_fields_idx ON embed_fields (embed_id);

CREATE TABLE IF NOT EXISTS attachments
(
    message_id   bigint NOT NULL,
    content_type text,
    ephemeral    bool   NOT NULL,
    filename     text   NOT NULL,
    description  text,
    height       bigint,
    id           bigint PRIMARY KEY,
    proxy_url    text   NOT NULL,
    size         bigint NOT NULL,
    url          text   NOT NULL,
    width        bigint
);
CREATE INDEX IF NOT EXISTS attachments_message_idx ON attachments (message_id);

CREATE TABLE IF NOT EXISTS presences
(
    guild_id bigint   NOT NULL,
    status   smallint NOT NULL,
    "user"   bigint   NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS presences_idx ON presences (guild_id, "user");
CREATE INDEX IF NOT EXISTS presences_user_idx ON presences ("user");

CREATE TABLE IF NOT EXISTS activities
(
    user_id            bigint   NOT NULL,
    guild_id           bigint   NOT NULL,
    application_id     bigint,
    asset_large_image  text,
    asset_large_text   text,
    asset_small_image  text,
    asset_small_text   text,
    created_at         bigint,
    details            text,
    emoji_animated     bool,
    emoji_name         text,
    emoji_id           text,
    flags              bigint,
    id                 text,
    instance           bool,
    kind               smallint NOT NULL,
    name               text     NOT NULL,
    party_id           text,
    party_size_current bigint,
    party_size_max     bigint,
    state              text,
    timestamp_end      bigint,
    timestamp_start    bigint,
    url                text
);
CREATE INDEX IF NOT EXISTS activities_user_idx ON activities (user_id);

CREATE TABLE IF NOT EXISTS reactions
(
    channel_id bigint NOT NULL,
    emoji      text   NOT NULL,
    guild_id   bigint,
    message_id bigint NOT NULL,
    user_id    bigint NOT NULL
);
CREATE INDEX IF NOT EXISTS reactions_idx ON reactions (message_id);

CREATE TABLE IF NOT EXISTS roles
(
    guild_id                bigint NOT NULL,
    user_id                 bigint,
    color                   bigint NOT NULL,
    hoist                   bool   NOT NULL,
    icon                    char(16),
    id                      bigint NOT NULL,
    managed                 bool   NOT NULL,
    mentionable             bool   NOT NULL,
    name                    text   NOT NULL,
    permissions             bigint NOT NULL,
    position                bigint NOT NULL,
    tags_bot_id             bigint,
    tags_integration_id     bigint,
    tags_premium_subscriber bool,
    unicode_emoji           text
);
CREATE UNIQUE INDEX IF NOT EXISTS roles_idx ON roles (user_id, id);
CREATE INDEX IF NOT EXISTS roles_guild_id_idx ON roles (guild_id);
CREATE INDEX IF NOT EXISTS roles_user_id_idx ON roles (user_id);
CREATE INDEX IF NOT EXISTS roles_name_idx ON roles (name);

CREATE TABLE IF NOT EXISTS stage_instances
(
    channel_id               bigint   NOT NULL,
    guild_id                 bigint   NOT NULL,
    guild_scheduled_event_id bigint,
    id                       bigint PRIMARY KEY,
    privacy_level            smallint NOT NULL,
    topic                    text     NOT NULL
);
CREATE INDEX IF NOT EXISTS stage_instances_guild_id_idx ON stage_instances (guild_id);