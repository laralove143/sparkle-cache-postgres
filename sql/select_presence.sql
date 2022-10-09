SELECT guild_id,
       status,
       "user"
FROM presences
WHERE user = $1