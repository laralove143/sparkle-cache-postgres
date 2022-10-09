use twilight_model::{channel::StageInstance, id::Id};

use crate::model::privacy_level_from_i16;

#[derive(Clone, Debug)]
pub struct QueriedStageInstance {
    pub channel_id: i64,
    pub guild_id: i64,
    pub guild_scheduled_event_id: Option<i64>,
    pub id: i64,
    pub privacy_level: i16,
    pub topic: String,
}

impl From<QueriedStageInstance> for StageInstance {
    fn from(stage_instance: QueriedStageInstance) -> Self {
        Self {
            channel_id: Id::new(stage_instance.channel_id as u64),
            guild_id: Id::new(stage_instance.guild_id as u64),
            guild_scheduled_event_id: stage_instance
                .guild_scheduled_event_id
                .map(|id| Id::new(id as u64)),
            id: Id::new(stage_instance.id as u64),
            privacy_level: privacy_level_from_i16(stage_instance.privacy_level),
            topic: stage_instance.topic,
        }
    }
}
