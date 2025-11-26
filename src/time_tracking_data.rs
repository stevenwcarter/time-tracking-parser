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

    pub fn validate_entries(&mut self, entries: &[TimeEntry]) {
        // Check for potential time order issues (duration > 6 hours or large gaps)
        self.validate_durations(entries);

        // Check for large gaps between consecutive entries that might indicate wrong order
        self.validate_dead_time(entries);
    }

    fn validate_durations(&mut self, entries: &[TimeEntry]) {
        for entry in entries {
            let duration = entry.duration_minutes();
            if duration > 8 * 60 {
                self.warnings.push(format!(
                "Time period {}-{} appears to be longer than 8 hours. Input may not be in correct order.",
                format_time(&entry.start),
                format_time(&entry.end)
            ));
            }
        }
    }

    fn validate_dead_time(&mut self, entries: &[TimeEntry]) {
        entries.windows(2).for_each(|chunk| {
            if let [first, second] = chunk {
                let gap = first.end.chronological_duration_minutes(&second.start);
                if gap > 6 * 60 {
                    self.warnings.push(format!(
                    "Gap from {} to {} appears to be longer than 6 hours. Input may not be in correct order.",
                    format_time(&first.end),
                    format_time(&second.start)
                ));
                }
            }
        });
    }
}
