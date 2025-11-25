use super::*;

/// Represents a time period with associated project and notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub start: Time,
    pub end: Time,
    pub project: String,
    pub notes: Vec<String>,
}

impl TimeEntry {
    pub fn duration_minutes(&self) -> u32 {
        self.start.duration_minutes(&self.end) as u32
    }
}
