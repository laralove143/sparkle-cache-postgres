DELETE
FROM reactions
WHERE message_id = $1
  AND user_id = $2
  AND emoji = $3;