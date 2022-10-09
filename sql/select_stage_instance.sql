SELECT channel_id,
       guild_id,
       guild_scheduled_event_id,
       id,
       privacy_level,
       topic
FROM stage_instances
WHERE id = $1