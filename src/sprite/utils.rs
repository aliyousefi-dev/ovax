

/// Formats seconds into a human-readable HH:MM:SS.ms string.
pub(crate) fn format_human_time(seconds_f64: f64) -> String {
    let total_secs_u64 = seconds_f64.floor() as u64;
    let hours = total_secs_u64 / 3600;
    let minutes = (total_secs_u64 % 3600) / 60;
    let seconds = total_secs_u64 % 60;
    let millis = ((seconds_f64 - total_secs_u64 as f64) * 1000.0).round() as u32;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}
