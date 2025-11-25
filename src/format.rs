use super::{time::*, time_tracking_data::*};

pub fn format_time_option(time: Option<&Time>, fallback: &str) -> String {
    if time.is_none() {
        return fallback.to_owned();
    }
    let time = time.unwrap();
    if time.minute == 0 {
        if time.hour == 0 {
            "00:00".to_owned()
        } else {
            format!("{:02}:00", time.hour)
        }
    } else {
        format!("{:02}:{:02}", time.hour, time.minute)
    }
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
