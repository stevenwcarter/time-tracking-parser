use std::{fmt::Display, str::FromStr};

use super::*;

mod hour;
mod minute;
pub use hour::Hour;
pub use minute::Minute;

/// Represents a time in 12-hour format (no AM/PM needed as per requirements)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub hour: Hour,
    pub minute: Minute,
}

impl Time {
    pub fn from_strings(hour: &str, minute: &str) -> Result<Self, String> {
        let hour: Hour = hour.parse()?;
        let minute: Minute = minute.parse()?;
        Ok(Time { hour, minute })
    }
    pub fn new(hour: u8, minute: u8) -> Result<Self, String> {
        if !(1..=12).contains(&hour) {
            return Err(format!("Hour must be between 1 and 12, got {hour}"));
        }
        if minute > 59 {
            return Err(format!("Minute must be between 0 and 59, got {minute}"));
        }
        let hour: Hour = hour.try_into()?;
        let minute: Minute = minute.try_into()?;
        Ok(Time { hour, minute })
    }

    /// Convert time to minutes since midnight (assuming 12-hour format)
    pub fn to_minutes(&self) -> u16 {
        let hour_24 = if self.hour == 12 { 0 } else { self.hour.get() };
        (hour_24 as u16 * 60) + self.minute.get() as u16
    }

    /// Calculate duration in minutes between two times
    /// This assumes both times are in the same 12-hour period
    pub fn duration_minutes(&self, end: &Time) -> i32 {
        let start_mins = self.to_minutes() as i32;
        let end_mins = end.to_minutes() as i32;

        if end_mins >= start_mins {
            end_mins - start_mins
        } else {
            // Cross noon/midnight - add 12 hours
            (12 * 60) + end_mins - start_mins
        }
    }

    /// Calculate duration in minutes between two times assuming chronological order
    /// If end time appears "earlier" than start time, assume it's in the next 12-hour period
    pub fn chronological_duration_minutes(&self, end: &Time) -> i32 {
        let start_mins = self.to_minutes() as i32;
        let end_mins = end.to_minutes() as i32;

        if end_mins > start_mins {
            end_mins - start_mins
        } else if end_mins == start_mins {
            // Same time = no gap for gap calculations, but 12 hours for individual entries
            0
        } else {
            // Assume we've crossed into the next 12-hour period
            (12 * 60) - start_mins + end_mins
        }
    }

    /// Format time as hours and minutes
    pub fn format_duration_minutes(minutes: u32) -> String {
        let hours = minutes / 60;
        let mins = minutes % 60;
        format!("{hours}:{mins:02}")
    }

    /// Format time as decimal hours
    pub fn format_duration_decimal(minutes: u32) -> String {
        let hours = minutes as f32 / 60.0;
        format!("{hours:.2}")
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = self.to_minutes() as u32;
        if minutes == 60 {
            write!(f, "1 hour")
        } else {
            write!(f, "{} hours", Self::format_duration_decimal(minutes))
        }
    }
}
