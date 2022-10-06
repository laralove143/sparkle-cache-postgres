INSERT INTO emojis (guild_id, animated, available, id, managed, name, require_colons, "user")
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (id) DO UPDATE SET (guild_id, animated, available, id, managed, name, require_colons,
                                "user") = ($1, $2, $3, $4, $5, $6, $7, $8)
