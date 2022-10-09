SELECT guild_id,
       user_id,
       color,
       hoist,
       icon,
       id,
       managed,
       mentionable,
       name,
       permissions,
       position,
       tags_bot_id,
       tags_integration_id,
       tags_premium_subscriber,
       unicode_emoji
FROM roles
WHERE user_id = $1
  AND guild_id = $2