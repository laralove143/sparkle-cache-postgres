SELECT channel_id,
       allow,
       deny,
       id,
       kind
FROM permission_overwrites
WHERE channel_id = $1