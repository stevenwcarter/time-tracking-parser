use super::time::*;

pub fn format_time_option(time: Option<&Time>, fallback: &str) -> String {
    if time.is_none() {
        return fallback.to_owned();
    }
    let time = time.unwrap();
    if time.hour == 0 {
        format!("00:{}", time.minute)
    } else {
        format!("{}:{}", time.hour, time.minute)
    }
}

/// Helper function to format a Time struct as a string
pub fn format_time(time: &Time) -> String {
    format!("{}:{}", time.hour, time.minute)
}
