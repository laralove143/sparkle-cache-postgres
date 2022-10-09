SELECT message_id,
       content_type,
       ephemeral,
       filename,
       description,
       height,
       id,
       proxy_url,
       size,
       url,
       width
FROM attachments
WHERE message_id = $1