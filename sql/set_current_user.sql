INSERT INTO "current_user" (accent_color,
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
                            verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
ON CONFLICT (id) DO UPDATE SET (accent_color, avatar, banner, bot, discriminator, email, flags, id, locale, mfa_enabled,
                                name, premium_type, public_flags,
                                verified) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)