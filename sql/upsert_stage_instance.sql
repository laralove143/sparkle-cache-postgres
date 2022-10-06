INSERT INTO stage_instances (channel_id, guild_id, guild_scheduled_event_id, id, privacy_level, topic)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (id)
    DO UPDATE SET (channel_id, guild_id, guild_scheduled_event_id, id, privacy_level, topic) = ($1, $2, $3, $4, $5, $6)
