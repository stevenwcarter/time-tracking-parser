use time_tracking_parser::*;

fn main() {
    let input = r#"
11:45-12:15 code1
- Comment explaining what you did
12:15-1:30 code2
- Comment about what you were doing
1:30-2 code1
2-4 code3
"#;

    println!("Parsing time tracking data...\n");
    let data = parse_time_tracking_data(input, None, None);

    if !data.warnings.is_empty() {
        println!("Warnings:");
        for warning in &data.warnings {
            println!("  - {warning}");
        }
        println!();
    }

    println!("=== FORMATTED OUTPUT ===");
    println!("{}", generate_sample_output(&data));

    println!("=== JSON OUTPUT ===");
    match data.to_json_pretty() {
        Ok(json) => println!("{json}"),
        Err(e) => println!("Error serializing to JSON: {e}"),
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
