INSERT INTO attachments (message_id, content_type, ephemeral, filename, description, height, id, proxy_url, size, url,
                         width)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (id)
    DO UPDATE SET (message_id, content_type, ephemeral, filename, description, height, id, proxy_url, size, url,
                   width) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
