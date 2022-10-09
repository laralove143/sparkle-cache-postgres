#![allow(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::fallible_impl_from,
    clippy::struct_excessive_bools
)]

use twilight_model::{
    channel::stage_instance::PrivacyLevel, gateway::presence::Status, user::PremiumType,
};

/// Activity related models
pub mod activity;
/// Attachment related models
pub mod attachment;
/// Channel related models
pub mod channel;
/// Current user related models
pub mod current_user;
/// Embed related models
pub mod embed;
/// Emoji related models
pub mod emoji;
/// Guild related models
pub mod guild;
/// Member related models
pub mod member;
/// Message related models
pub mod message;
/// Permission overwrite related models
pub mod permission_overwrite;
/// Presence related models
pub mod presence;
/// Reaction related models
pub mod reaction;
/// Role related models
pub mod role;
/// Stage instance related models
pub mod stage_instance;
/// Sticker related models
pub mod sticker;

/// Return a unique integer based on the variant of the status
pub const fn status_as_i16(status: Status) -> i16 {
    match status {
        Status::DoNotDisturb => 0,
        Status::Idle => 1,
        Status::Invisible => 2,
        Status::Offline => 3,
        Status::Online => 4,
    }
}

/// Return a [`twilight_model::gateway::presence::Status`] based on the integer
/// returned by [`status_as_i16`]
pub const fn status_from_i16(repr: i16) -> Status {
    match repr {
        0 => Status::DoNotDisturb,
        1 => Status::Idle,
        2 => Status::Invisible,
        3 => Status::Offline,
        4 => Status::Online,
        _ => panic!("The given integer is too big to represent Status"),
    }
}

/// Return a [`twilight_model::user::PremiumType`] based on the enum's integer
/// representation
pub const fn premium_type_from_i16(repr: i16) -> PremiumType {
    match repr {
        0 => PremiumType::None,
        1 => PremiumType::NitroClassic,
        2 => PremiumType::Nitro,
        _ => panic!("The given integer is too big to represent PremiumType"),
    }
}

/// Return a [`twilight_model::channel::stage_instance::PrivacyLevel`] based on
/// the enum's integer representation
pub const fn privacy_level_from_i16(repr: i16) -> PrivacyLevel {
    match repr {
        0 => PrivacyLevel::GuildOnly,
        _ => panic!("The given integer is too big to represent PrivacyLevel"),
    }
}
