use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a time in 12-hour format (no AM/PM needed as per requirements)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn new(hour: u8, minute: u8) -> Result<Self, String> {
        if !(1..=12).contains(&hour) {
            return Err(format!("Hour must be between 1 and 12, got {hour}"));
        }
        if minute > 59 {
            return Err(format!("Minute must be between 0 and 59, got {minute}"));
        }
        Ok(Time { hour, minute })
    }

    /// Convert time to minutes since midnight (assuming 12-hour format)
    pub fn to_minutes(&self) -> u16 {
        let hour_24 = if self.hour == 12 { 0 } else { self.hour };
        (hour_24 as u16 * 60) + self.minute as u16
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
        if mins == 0 {
            format!("{hours}:00")
        } else {
            format!("{hours}:{mins:02}")
        }
    }

    /// Format time as decimal hours
    pub fn format_duration_decimal(minutes: u32) -> String {
        let hours = minutes as f32 / 60.0;
        format!("{hours:.2}")
    }
}

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

/// Represents aggregated project data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSummary {
    pub name: String,
    pub total_minutes: u32,
    pub notes: Vec<String>,
}

impl ProjectSummary {
    pub fn new(name: String) -> Self {
        ProjectSummary {
            name,
            total_minutes: 0,
            notes: Vec::new(),
        }
    }

    pub fn add_time(&mut self, minutes: u32) {
        self.total_minutes += minutes;
    }

    pub fn add_notes(&mut self, notes: Vec<String>) {
        self.notes.extend(notes);
    }
}

/// Main struct holding all parsed time tracking data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
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

/// Parse a time string like "7:30" or "7"
fn parse_time(time_str: &str) -> Result<Time, String> {
    let parts: Vec<&str> = time_str.split(':').collect();

    match parts.len() {
        1 => {
            let hour: u8 = parts[0]
                .parse()
                .map_err(|_| format!("Invalid hour: {}", parts[0]))?;
            Time::new(hour, 0)
        }
        2 => {
            let hour: u8 = parts[0]
                .parse()
                .map_err(|_| format!("Invalid hour: {}", parts[0]))?;
            let minute: u8 = parts[1]
                .parse()
                .map_err(|_| format!("Invalid minute: {}", parts[1]))?;
            Time::new(hour, minute)
        }
        _ => Err(format!("Invalid time format: {time_str}")),
    }
}

/// Parse a time range like "7:30-8" or "8-8:30"
fn parse_time_range(range_str: &str) -> Result<(Time, Time), String> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid time range format: {range_str}"));
    }

    let start = parse_time(parts[0].trim())?;
    let end = parse_time(parts[1].trim())?;

    Ok((start, end))
}

/// Main parsing function
pub fn parse_time_tracking_data(input: &str) -> TimeTrackingData {
    let mut data = TimeTrackingData::new();
    let mut entries = Vec::new();
    let mut current_entry: Option<TimeEntry> = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('-') {
            // This is a note line
            if let Some(ref mut entry) = current_entry {
                if let Some(note) = line.strip_prefix('-') {
                    entry.notes.push(note.trim().to_string());
                }
            }
        } else {
            // Save previous entry if exists
            if let Some(entry) = current_entry.take() {
                entries.push(entry);
            }

            // Parse new time entry
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                data.warnings
                    .push(format!("Line missing project name: {line}"));
                // Don't continue here - we want to parse the time range for dead time calculation
                // but we won't create a valid entry
                if let Ok((start, end)) = parse_time_range(parts[0]) {
                    // Create a temporary entry just for dead time calculation
                    // but don't save it as a real entry
                    current_entry = Some(TimeEntry {
                        start,
                        end,
                        project: String::new(), // Empty project name indicates invalid entry
                        notes: Vec::new(),
                    });
                }
                continue;
            }

            match parse_time_range(parts[0]) {
                Ok((start, end)) => {
                    let project = parts[1].trim().to_string();
                    current_entry = Some(TimeEntry {
                        start,
                        end,
                        project,
                        notes: Vec::new(),
                    });
                }
                Err(e) => {
                    data.warnings
                        .push(format!("Error parsing time range '{}': {}", parts[0], e));
                }
            }
        }
    }

    // Don't forget the last entry
    if let Some(entry) = current_entry {
        entries.push(entry);
    }

    // Check for potential time order issues (duration > 6 hours or large gaps)
    for entry in &entries {
        let duration = entry.duration_minutes();
        if duration > 6 * 60 {
            data.warnings.push(format!(
                "Time period {}-{} appears to be longer than 6 hours. Input may not be in correct order.",
                format_time(&entry.start),
                format_time(&entry.end)
            ));
        }
    }

    // Check for large gaps between consecutive entries that might indicate wrong order
    let mut last_end: Option<&Time> = None;
    for entry in &entries {
        if let Some(prev_end) = last_end {
            let gap = prev_end.chronological_duration_minutes(&entry.start);
            if gap > 6 * 60 {
                data.warnings.push(format!(
                    "Gap from {} to {} appears to be longer than 6 hours. Input may not be in correct order.",
                    format_time(prev_end),
                    format_time(&entry.start)
                ));
            }
        }
        last_end = Some(&entry.end);
    }

    // Calculate overall start and end times using all entries
    if !entries.is_empty() {
        data.start_time = Some(entries.first().unwrap().start.clone());
        data.end_time = Some(entries.last().unwrap().end.clone());
    }

    // Calculate total working time using all entries (including ones without project names)
    let mut total_minutes = 0;
    for entry in &entries {
        total_minutes += entry.duration_minutes();
    }

    // Calculate dead time using all entries (reuse the gap calculation)
    let mut last_end: Option<&Time> = None;
    for entry in &entries {
        if let Some(prev_end) = last_end {
            let gap = prev_end.chronological_duration_minutes(&entry.start);
            if gap > 0 {
                // Count ALL gaps as dead time, regardless of size
                data.dead_time_minutes += gap as u32;
            }
        }
        last_end = Some(&entry.end);
    }

    data.total_minutes = total_minutes;

    // Aggregate by project using only entries with valid project names
    let mut project_map: HashMap<String, ProjectSummary> = HashMap::new();

    for entry in &entries {
        // Skip entries without project names for project aggregation
        if entry.project.is_empty() {
            continue;
        }

        let project_summary = project_map
            .entry(entry.project.clone())
            .or_insert_with(|| ProjectSummary::new(entry.project.clone()));

        project_summary.add_time(entry.duration_minutes());
        project_summary.add_notes(entry.notes.clone());
    }

    data.projects = project_map.into_values().collect();
    data.projects.sort_by(|a, b| a.name.cmp(&b.name));

    data
}

/// Helper function to format a Time struct as a string
pub fn format_time(time: &Time) -> String {
    if time.minute == 0 {
        format!("{}:00", time.hour)
    } else {
        format!("{}:{:02}", time.hour, time.minute)
    }
}

/// Generate sample output for testing/comparison (as requested)
pub fn generate_sample_output(data: &TimeTrackingData) -> String {
    let mut output = String::new();

    if let (Some(start), Some(end)) = (&data.start_time, &data.end_time) {
        output.push_str(&format!(
            "Start Time: {} End Time: {}\n",
            format_time(start),
            format_time(end)
        ));
    }

    output.push_str(&format!(
        "Total Working Time: {} ({} hrs)\n",
        Time::format_duration_minutes(data.total_minutes),
        Time::format_duration_decimal(data.total_minutes)
    ));

    output.push_str(&format!(
        "Total dead time: {} ({} hrs)\n",
        Time::format_duration_minutes(data.dead_time_minutes),
        Time::format_duration_decimal(data.dead_time_minutes)
    ));

    output.push('\n');

    for project in &data.projects {
        output.push_str(&format!(
            "Billing Code: {} - {} ({} hrs)\n",
            project.name,
            Time::format_duration_minutes(project.total_minutes),
            Time::format_duration_decimal(project.total_minutes)
        ));

        for note in &project.notes {
            output.push_str(&format!("- {note}\n"));
        }
        output.push('\n');
    }

    output
}

pub fn parse_time_data(input: &str) -> String {
    let data = parse_time_tracking_data(input);
    generate_sample_output(&data)
}

pub fn parse_time_data_to_json(input: &str) -> String {
    let data = parse_time_tracking_data(input);
    data.to_json()
        .unwrap_or_else(|e| format!("Error serializing to JSON: {e}"))
}

pub fn parse_time_data_to_json_pretty(input: &str) -> String {
    let data = parse_time_tracking_data(input);
    data.to_json_pretty()
        .unwrap_or_else(|e| format!("Error serializing to JSON: {e}"))
}
