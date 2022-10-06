DELETE
FROM activities
WHERE guild_id = $1
  AND user_id = $2