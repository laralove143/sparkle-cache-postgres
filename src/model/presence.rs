use twilight_model::gateway::presence::Status;

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
