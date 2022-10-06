DELETE
FROM reactions
WHERE message_id = $1
  AND emoji = $2;