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
WHERE message_id = $1