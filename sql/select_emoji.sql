SELECT guild_id,
       animated,
       available,
       id,
       managed,
       name,
       require_colons,
       "user"
FROM emojis
WHERE id = $1