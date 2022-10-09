SELECT accent_color,
       avatar,
       banner,
       bot,
       discriminator,
       email,
       flags,
       id,
       locale,
       mfa_enabled,
       name,
       premium_type,
       public_flags,
       verified
FROM "current_user"
LIMIT 1