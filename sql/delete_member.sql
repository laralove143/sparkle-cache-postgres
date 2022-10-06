DELETE
FROM members
WHERE id = $1
  AND guild_id = $2;