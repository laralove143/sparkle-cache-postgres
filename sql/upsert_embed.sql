INSERT INTO embeds (id, message_id, author_icon_url, author_name, author_proxy_icon_url, author_url, color, description,
                    footer_icon_url, footer_proxy_icon_url, footer_text, image_height, image_proxy_url, image_url,
                    image_width, kind, provider_name, provider_url, thumbnail_height, thumbnail_proxy_url,
                    thumbnail_url, thumbnail_width, timestamp, title, url, video_height, video_proxy_url, video_url,
                    video_width)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
        $25, $26, $27, $28, $29)
ON CONFLICT (id)
    DO UPDATE SET (id, message_id, author_icon_url, author_name, author_proxy_icon_url, author_url, color, description,
                   footer_icon_url, footer_proxy_icon_url, footer_text, image_height, image_proxy_url, image_url,
                   image_width, kind, provider_name, provider_url, thumbnail_height, thumbnail_proxy_url,
                   thumbnail_url, thumbnail_width, timestamp, title, url, video_height, video_proxy_url, video_url,
                   video_width) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
                                   $20, $21, $22, $23, $24,
                                   $25, $26, $27, $28, $29)
