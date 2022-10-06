INSERT INTO members (guild_avatar, communication_disabled_until, deaf, guild_id, joined_at, mute, nick, pending,
                     premium_since, accent_color, avatar, banner, bot, discriminator, flags, id, locale, mfa_enabled,
                     name, premium_type, public_flags, system)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
ON CONFLICT (id)
    DO UPDATE SET (guild_avatar, communication_disabled_until, deaf, guild_id, joined_at, mute, nick, pending,
                   premium_since, accent_color, avatar, banner, bot, discriminator, flags, id, locale, mfa_enabled,
                   name, premium_type, public_flags, system) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                                                                $14, $15, $16, $17, $18, $19, $20, $21, $22)
