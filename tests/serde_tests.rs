use time_tracking_parser::*;

#[test]
fn test_serde_serialization() {
    let input = r#"7:30-8 someproject
8-8:30 general
- discussing staffing with colleague"#;

    let data = parse_time_tracking_data(input, None, None);

    // Test JSON serialization
    let json = data.to_json().expect("Should serialize to JSON");
    assert!(json.contains(&format!("\"total_minutes\":{}", data.total_minutes)));
    assert!(json.contains("\"someproject\""));
    assert!(json.contains("\"general\""));

    // Test pretty JSON serialization
    let pretty_json = data
        .to_json_pretty()
        .expect("Should serialize to pretty JSON");
    assert!(pretty_json.contains(&format!("\"total_minutes\": {}", data.total_minutes)));
    assert!(pretty_json.contains("\n"));

    // Test deserialization
    let deserialized = TimeTrackingData::from_json(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.total_minutes, data.total_minutes);
    assert_eq!(deserialized.dead_time_minutes, data.dead_time_minutes);
    assert_eq!(deserialized.projects.len(), data.projects.len());
    assert_eq!(deserialized.warnings.len(), data.warnings.len());

    // Test that start and end times are preserved
    assert_eq!(deserialized.start_time, data.start_time);
    assert_eq!(deserialized.end_time, data.end_time);
}

#[test]
fn test_serde_round_trip() {
    let input = r#"7:30-8 someproject
- investigated BTS-446
8-8:30 general
- discussing staffing with colleague
11-12 other-project
- tech connect"#;

    let original_data = parse_time_tracking_data(input, None, None);

    // Serialize to JSON
    let json = original_data.to_json().expect("Should serialize");

    // Deserialize back
    let restored_data = TimeTrackingData::from_json(&json).expect("Should deserialize");

    // Verify all fields match
    assert_eq!(restored_data.total_minutes, original_data.total_minutes);
    assert_eq!(
        restored_data.dead_time_minutes,
        original_data.dead_time_minutes
    );
    assert_eq!(restored_data.warnings, original_data.warnings);
    assert_eq!(restored_data.start_time, original_data.start_time);
    assert_eq!(restored_data.end_time, original_data.end_time);
    assert_eq!(restored_data.projects.len(), original_data.projects.len());

    // Verify projects match
    for (original_project, restored_project) in
        original_data.projects.iter().zip(&restored_data.projects)
    {
        assert_eq!(original_project.name, restored_project.name);
        assert_eq!(
            original_project.total_minutes,
            restored_project.total_minutes
        );
        assert_eq!(original_project.notes, restored_project.notes);
    }
}

#[test]
fn test_wasm_json_functions() {
    let input = r#"7:30-8 someproject
8-8:30 general"#;

    // Test WASM JSON function
    let json_output = parse_time_data_to_json(input, None, None);
    assert!(json_output.contains(&format!("\"total_minutes\":{}", 60)));
    assert!(!json_output.starts_with("Error"));

    // Test WASM pretty JSON function
    let pretty_json_output = parse_time_data_to_json_pretty(input, None, None);
    assert!(pretty_json_output.contains(&format!("\"total_minutes\": {}", 60)));
    assert!(pretty_json_output.contains("\n"));
    assert!(!pretty_json_output.starts_with("Error"));

    // Verify we can deserialize the WASM output
    let parsed_data =
        TimeTrackingData::from_json(&json_output).expect("Should parse WASM JSON output");
    assert_eq!(parsed_data.total_minutes, 60);
    assert_eq!(parsed_data.projects.len(), 2);
}

#[test]
fn test_json_with_warnings() {
    let input = r#"7-8 project1
3-4 project2"#; // This should generate a warning

    let data = parse_time_tracking_data(input, None, None);
    assert!(!data.warnings.is_empty());

    let json = data.to_json().expect("Should serialize even with warnings");
    let restored = TimeTrackingData::from_json(&json).expect("Should deserialize");

    assert_eq!(restored.warnings, data.warnings);
    assert!(!restored.warnings.is_empty());
}
