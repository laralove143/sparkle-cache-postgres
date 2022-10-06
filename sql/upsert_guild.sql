INSERT INTO guilds (afk_channel_id, afk_timeout, application_id, banner, default_message_notifications, description,
                    discovery_splash, explicit_content_filter, features, icon, id, joined_at, large, max_members,
                    max_presences, max_video_channel_users, mfa_level, name, nsfw_level, owner_id, owner, permissions,
                    preferred_locale, premium_progress_bar_enabled, premium_subscription_count, premium_tier,
                    rules_channel_id, splash, system_channel_flags, system_channel_id, unavailable, vanity_url_code,
                    verification_level, widget_channel_id, widget_enabled)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
        $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35)
ON CONFLICT (id)
    DO UPDATE SET (afk_channel_id, afk_timeout, application_id, banner, default_message_notifications, description,
                   discovery_splash, explicit_content_filter, features, icon, id, joined_at, large, max_members,
                   max_presences, max_video_channel_users, mfa_level, name, nsfw_level, owner_id, owner, permissions,
                   preferred_locale, premium_progress_bar_enabled, premium_subscription_count, premium_tier,
                   rules_channel_id, splash, system_channel_flags, system_channel_id, unavailable, vanity_url_code,
                   verification_level, widget_channel_id, widget_enabled) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                                                                             $11, $12, $13, $14, $15, $16, $17, $18,
                                                                             $19, $20, $21, $22, $23, $24,
                                                                             $25, $26, $27, $28, $29, $30, $31, $32,
                                                                             $33, $34, $35)
