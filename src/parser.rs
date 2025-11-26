use std::collections::HashMap;

use std::sync::OnceLock;
use strip_prefix_suffix_sane::StripPrefixSuffixSane;

use super::*;

static TIME_REGEX: OnceLock<regex::Regex> = OnceLock::new();

/// Parse a time string like "7:30" or "7"
fn parse_time(time_str: &str) -> Result<Time, String> {
    let mut parts = time_str.split(':');

    let hour = parts
        .next()
        .ok_or_else(|| format!("Invalid time format: {time_str}"))?;
    let minute = parts.next().unwrap_or("00");

    Time::from_strings(hour, minute)
}

/// Parse a time range like "7:30-8" or "8-8:30"
fn parse_time_range(range_str: &str) -> Result<(Time, Time), String> {
    let (start, end) = range_str
        .split_once('-')
        .ok_or_else(|| format!("Invalid time range format: {range_str}"))?;

    let start = parse_time(start.trim())?;
    let end = parse_time(end.trim())?;

    Ok((start, end))
}

/// Check if a line looks like a time tracking entry (e.g., "10-2 project" or "10:30-3 project")
/// This includes lines that have the time pattern but might be missing the project name
fn is_time_tracking_line(line: &str, prefix: Option<&str>) -> bool {
    // Use regex to match time patterns like "10-2" or "10:30-3:45", with or without project name
    let regex = TIME_REGEX.get_or_init(|| {
        regex::Regex::new(r"^\d{1,2}(?::\d{2})?-\d{1,2}(?::\d{2})?")
            .expect("could not compile regex")
    });

    if let Some(prefix) = prefix {
        line.starts_with(prefix)
    } else {
        regex.is_match(line)
    }
}

/// Check if we should continue parsing (line starts with number, dash, or space)
fn should_continue_parsing(line: &str, suffix: Option<&str>) -> bool {
    if let Some(suffix) = suffix {
        !line.starts_with(suffix)
    } else {
        true
    }
}

/// Main parsing function
pub fn parse_time_tracking_data(
    input: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
) -> TimeTrackingData {
    let mut data = TimeTrackingData::new();
    let mut entries = Vec::new();
    let mut current_entry: Option<TimeEntry> = None;
    let mut parsing_started = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // If we haven't started parsing yet, look for the first time tracking line
        if !parsing_started {
            if is_time_tracking_line(line, prefix) {
                parsing_started = true;
                if prefix.is_some() {
                    continue; // Skip the prefix line
                }
            } else {
                continue; // Skip lines until we find a time tracking pattern
            }
        }

        // If we've started parsing, check if we should continue
        if parsing_started && !should_continue_parsing(line, suffix) {
            break; // Stop parsing when we hit a line that doesn't start with number, dash, or space
        }

        if !line.starts_with(char::is_numeric) && !line.is_empty() {
            if let Some(ref mut entry) = current_entry {
                entry.notes.push(
                    line.strip_prefix_sane("-")
                        .strip_prefix_sane("*")
                        .trim()
                        .to_string(),
                );
            }
        } else {
            // Save previous entry if exists
            if let Some(entry) = current_entry.take() {
                entries.push(entry);
            }

            // Parse new time entry
            let mut parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                data.warnings
                    .push(format!("Line missing project name: {line}"));
                parts.push("missing");
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
    data.validate_entries(&entries);

    // Calculate overall start and end times using all entries
    if !entries.is_empty() {
        data.start_time = Some(entries.first().unwrap().start);
        data.end_time = Some(entries.last().unwrap().end);
    }

    // Calculate total working time using all entries (including ones without project names)
    let mut total_minutes = 0;
    for entry in &entries {
        total_minutes += entry.duration_minutes();
    }

    // Calculate dead time using all entries (reuse the gap calculation)
    entries.windows(2).for_each(|chunk| {
        if let [first, second] = chunk {
            let gap = first.end.chronological_duration_minutes(&second.start);
            if gap > 0 {
                data.dead_time_minutes += gap;
            }
        }
    });

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

pub fn parse_time_data_to_json(input: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
    let data = parse_time_tracking_data(input, prefix, suffix);
    data.to_json()
        .unwrap_or_else(|e| format!("Error serializing to JSON: {e}"))
}

pub fn parse_time_data_to_json_pretty(
    input: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
) -> String {
    let data = parse_time_tracking_data(input, prefix, suffix);
    data.to_json_pretty()
        .unwrap_or_else(|e| format!("Error serializing to JSON: {e}"))
}
