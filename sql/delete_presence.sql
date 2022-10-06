DELETE
FROM presences
WHERE guild_id = $1
  AND "user" = $2