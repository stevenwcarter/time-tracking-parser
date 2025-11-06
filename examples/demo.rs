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
