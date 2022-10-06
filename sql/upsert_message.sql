INSERT INTO messages (activity_type, activity_party_id, application_cover_image, application_description,
                      application_icon, application_id, application_name, interaction_application_id, author,
                      channel_id, content, edited_timestamp, flags, guild_id, id, kind, mention_everyone, pinned,
                      reference_channel_id, reference_guild_id, reference_message_id, reference_fail_if_not_exists,
                      referenced_message, timestamp, thread, tts, webhook_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
        $25, $26, $27)
ON CONFLICT (id)
    DO UPDATE SET (activity_type, activity_party_id, application_cover_image, application_description,
                   application_icon, application_id, application_name, interaction_application_id, author,
                   channel_id, content, edited_timestamp, flags, guild_id, id, kind, mention_everyone, pinned,
                   reference_channel_id, reference_guild_id, reference_message_id, reference_fail_if_not_exists,
                   referenced_message, timestamp, thread, tts, webhook_id) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                                                                              $11, $12, $13, $14, $15, $16, $17, $18,
                                                                              $19, $20, $21, $22, $23, $24,
                                                                              $25, $26, $27)
