INSERT INTO stickers (message_id, available, description, format_type, guild_id, id, kind, name, pack_id, sort_value,
                      tags, user_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (id)
    DO UPDATE SET (available, description, format_type, guild_id, id, kind, name, pack_id, sort_value,
                   tags, user_id) = ($2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
