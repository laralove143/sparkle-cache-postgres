INSERT INTO activities (user_id, guild_id, application_id, asset_large_image, asset_large_text, asset_small_image,
                        asset_small_text, created_at, details, emoji_animated, emoji_name, emoji_id, flags, id,
                        instance, kind, name, party_id, party_size_current, party_size_max, state, timestamp_end,
                        timestamp_start, url)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)