DELETE
FROM stickers
WHERE guild_id = $1
  AND message_id IS NULL