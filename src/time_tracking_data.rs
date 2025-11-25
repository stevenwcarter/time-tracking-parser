use super::*;

/// Main struct holding all parsed time tracking data
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingData {
    pub total_minutes: u32,
    pub dead_time_minutes: u32,
    pub projects: Vec<ProjectSummary>,
    pub warnings: Vec<String>,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
}

impl TimeTrackingData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize the data to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize the data to pretty-formatted JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn formatted_start_time(&self) -> String {
        self.start_time
            .as_ref()
            .map_or("N/A".to_string(), format_time)
    }

    pub fn formatted_end_time(&self) -> String {
        self.end_time
            .as_ref()
            .map_or("N/A".to_string(), format_time)
    }

    pub fn formatted_total_minutes(&self) -> String {
        Time::format_duration_minutes(self.total_minutes)
    }
    pub fn formatted_dead_time_minutes(&self) -> String {
        Time::format_duration_minutes(self.dead_time_minutes)
    }
    pub fn formatted_total_decimal(&self) -> String {
        Time::format_duration_decimal(self.total_minutes)
    }
    pub fn formatted_dead_decimal(&self) -> String {
        Time::format_duration_decimal(self.dead_time_minutes)
    }
}
