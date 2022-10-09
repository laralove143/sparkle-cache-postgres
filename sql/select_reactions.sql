SELECT channel_id,
       emoji,
       guild_id,
       message_id,
       user_id
FROM reactions
WHERE message_id = $1