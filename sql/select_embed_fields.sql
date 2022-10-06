SELECT embed_id,
       inline,
       name,
       value
FROM embed_fields
WHERE embed_id = $1;