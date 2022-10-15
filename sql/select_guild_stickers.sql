SELECT message_id,
       available,
       description,
       format_type,
       guild_id,
       id,
       kind,
       name,
       pack_id,
       sort_value,
       tags,
       user_id
FROM stickers
WHERE id IN (SELECT DISTINCT id FROM stickers WHERE guild_id = $1)