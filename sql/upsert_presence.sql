INSERT INTO presences (guild_id, status, "user")
VALUES ($1, $2, $3)
ON CONFLICT (guild_id, "user")
    DO UPDATE SET (guild_id, status, "user") = ($1, $2, $3)
