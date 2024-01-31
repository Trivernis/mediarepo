use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};

/// Converts a system time timestamp to a NaiveDateTime object
pub fn system_time_to_naive_date_time(system_time: SystemTime) -> NaiveDateTime {
    let epoch_duration = system_time.duration_since(UNIX_EPOCH).unwrap();

    NaiveDateTime::from_timestamp_opt(
        epoch_duration.as_secs() as i64,
        epoch_duration.subsec_nanos(),
    )
    .unwrap()
}
