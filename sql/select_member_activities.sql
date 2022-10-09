SELECT user_id,
       guild_id,
       application_id,
       asset_large_image,
       asset_large_text,
       asset_small_image,
       asset_small_text,
       created_at,
       details,
       emoji_animated,
       emoji_name,
       emoji_id,
       flags,
       id,
       instance,
       kind,
       name,
       party_id,
       party_size_current,
       party_size_max,
       state,
       timestamp_end,
       timestamp_start,
       url
FROM activities
WHERE user_id = $1
LIMIT 1