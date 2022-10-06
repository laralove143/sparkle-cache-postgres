INSERT INTO channels (application_id, bitrate, default_auto_archive_duration, guild_id, icon, id, invitable, kind, name,
                      nsfw, owner_id, parent_id, position, rate_limit_per_user, rtc_region, thread_archived,
                      thread_auto_archive_duration, thread_archive_timestamp, thread_create_timestamp, thread_invitable,
                      thread_locked, topic, user_limit, video_quality_mode)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
ON CONFLICT (id)
    DO UPDATE SET (application_id, bitrate, default_auto_archive_duration, guild_id, icon, id, invitable, kind, name,
                   nsfw, owner_id, parent_id, position, rate_limit_per_user, rtc_region, thread_archived,
                   thread_auto_archive_duration, thread_archive_timestamp, thread_create_timestamp, thread_invitable,
                   thread_locked, topic, user_limit, video_quality_mode) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                                                                            $11, $12, $13, $14, $15, $16, $17, $18, $19,
                                                                            $20, $21, $22, $23, $24)
