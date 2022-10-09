SELECT application_id,
       bitrate,
       default_auto_archive_duration,
       guild_id,
       icon,
       id,
       invitable,
       kind,
       name,
       nsfw,
       owner_id,
       parent_id,
       position,
       rate_limit_per_user,
       rtc_region,
       thread_archived,
       thread_auto_archive_duration,
       thread_archive_timestamp,
       thread_create_timestamp,
       thread_invitable,
       thread_locked,
       topic,
       user_limit,
       video_quality_mode
FROM channels
WHERE id = $1