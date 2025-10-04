use time_tracking_parser::*;

#[test]
fn test_parse_basic_time_tracking() {
    let input = r#"7:30-8 someproject
8-8:30 admin
- discussing staffing with colleague
8:30-11 someproject
- investigated issue, pushed PR
- pushed potential fix for component /build url handling
11-12 other-project
- tech connect
12-12:30 admin
- 1:1 w/ coworker
12:30-2:30 someproject
- discussing work items and how to complete"#;

    let data = parse_time_tracking_data(input);

    // Check basic totals
    assert_eq!(data.total_minutes, 420); // 7 hours
    assert_eq!(data.dead_time_minutes, 0);
    assert!(data.warnings.is_empty());

    // Check start and end times
    assert!(data.start_time.is_some());
    assert!(data.end_time.is_some());
    let start = data.start_time.unwrap();
    let end = data.end_time.unwrap();
    assert_eq!(start.hour, 7);
    assert_eq!(start.minute, 30);
    assert_eq!(end.hour, 2);
    assert_eq!(end.minute, 30);

    // Check projects
    assert_eq!(data.projects.len(), 3);

    // Find and verify each project
    let someproject = data
        .projects
        .iter()
        .find(|p| p.name == "someproject")
        .unwrap();
    assert_eq!(someproject.total_minutes, 300); // 5 hours
    assert_eq!(someproject.notes.len(), 3);
    assert!(
        someproject
            .notes
            .contains(&"investigated issue, pushed PR".to_string())
    );
    assert!(
        someproject
            .notes
            .contains(&"pushed potential fix for component /build url handling".to_string())
    );
    assert!(
        someproject
            .notes
            .contains(&"discussing work items and how to complete".to_string())
    );

    let admin = data.projects.iter().find(|p| p.name == "admin").unwrap();
    assert_eq!(admin.total_minutes, 60); // 1 hour
    assert_eq!(admin.notes.len(), 2);
    assert!(
        admin
            .notes
            .contains(&"discussing staffing with colleague".to_string())
    );
    assert!(admin.notes.contains(&"1:1 w/ coworker".to_string()));

    let thomson = data
        .projects
        .iter()
        .find(|p| p.name == "other-project")
        .unwrap();
    assert_eq!(thomson.total_minutes, 60); // 1 hour
    assert_eq!(thomson.notes.len(), 1);
    assert!(thomson.notes.contains(&"tech connect".to_string()));
}

#[test]
fn test_parse_with_gaps() {
    let input = r#"7-8 project1
9-10 project2"#;

    let data = parse_time_tracking_data(input);

    assert_eq!(data.total_minutes, 120); // 2 hours
    assert_eq!(data.dead_time_minutes, 60); // 1 hour gap
    assert!(data.warnings.is_empty());
}

#[test]
fn test_parse_missing_project_name() {
    let input = r#"7-8
9-10 project2"#;

    let data = parse_time_tracking_data(input);

    assert_eq!(data.warnings.len(), 1);
    assert!(data.warnings[0].contains("Line missing project name"));
    assert_eq!(data.projects.len(), 1);
}

#[test]
fn test_parse_long_duration_warning() {
    let input = r#"2-3 project1
1-2 project2"#; // Gap from 3 to 1 should be 10 hours, but this suggests wrong order

    let data = parse_time_tracking_data(input);

    // Debug: let's see what warnings we actually get
    println!("Warnings: {:?}", data.warnings);
    println!("Dead time minutes: {}", data.dead_time_minutes);
    for (i, entry) in data.projects.iter().enumerate() {
        println!(
            "Project {}: {} - {} minutes",
            i, entry.name, entry.total_minutes
        );
    }

    // Let's manually check the gap calculation
    use time_tracking_parser::Time;
    let time_3 = Time::new(3, 0).unwrap();
    let time_1 = Time::new(1, 0).unwrap();
    let gap = time_3.chronological_duration_minutes(&time_1);
    println!("Gap from 3:00 to 1:00: {gap} minutes");

    // This should trigger a warning because going from 3 to 1 suggests a 22-hour gap
    assert!(!data.warnings.is_empty());
    assert!(
        data.warnings
            .iter()
            .any(|w| w.contains("longer than 6 hours"))
    );
}

#[test]
fn test_parse_hour_only_format() {
    let input = r#"7-8 project1
8-9 project2"#;

    let data = parse_time_tracking_data(input);

    assert_eq!(data.total_minutes, 120); // 2 hours
    assert_eq!(data.projects.len(), 2);
    assert!(data.warnings.is_empty());
}

#[test]
fn test_parse_mixed_time_formats() {
    let input = r#"7:30-8 project1
8-8:15 project2"#;

    let data = parse_time_tracking_data(input);

    assert_eq!(data.total_minutes, 45); // 30 + 15 minutes
    assert_eq!(data.projects.len(), 2);
    assert!(data.warnings.is_empty());
}

#[test]
fn test_parse_notes_without_time_entry() {
    let input = r#"- orphaned note
7-8 project1
- real note"#;

    let data = parse_time_tracking_data(input);

    // Orphaned notes should be ignored
    assert_eq!(data.projects.len(), 1);
    let project = &data.projects[0];
    assert_eq!(project.notes.len(), 1);
    assert!(project.notes.contains(&"real note".to_string()));
}

#[test]
fn test_parse_empty_input() {
    let data = parse_time_tracking_data("");

    assert_eq!(data.total_minutes, 0);
    assert_eq!(data.dead_time_minutes, 0);
    assert_eq!(data.projects.len(), 0);
    assert!(data.warnings.is_empty());
    assert!(data.start_time.is_none());
    assert!(data.end_time.is_none());
}

#[test]
fn test_parse_invalid_time_format() {
    let input = r#"25:70-8 project1
7-26 project2
7:70-8 project3"#;

    let data = parse_time_tracking_data(input);

    assert!(data.warnings.len() >= 2); // Should have warnings for invalid times
    assert_eq!(data.projects.len(), 0); // No valid entries
}

#[test]
fn test_project_summary_aggregation() {
    let input = r#"7-8 project1
- note 1
9-10 project1
- note 2
11-12 project2
- note 3"#;

    let data = parse_time_tracking_data(input);

    assert_eq!(data.projects.len(), 2);

    let project1 = data.projects.iter().find(|p| p.name == "project1").unwrap();
    assert_eq!(project1.total_minutes, 120); // 2 hours
    assert_eq!(project1.notes.len(), 2);
    assert!(project1.notes.contains(&"note 1".to_string()));
    assert!(project1.notes.contains(&"note 2".to_string()));

    let project2 = data.projects.iter().find(|p| p.name == "project2").unwrap();
    assert_eq!(project2.total_minutes, 60); // 1 hour
    assert_eq!(project2.notes.len(), 1);
    assert!(project2.notes.contains(&"note 3".to_string()));
}

#[test]
fn test_generate_sample_output() {
    let input = r#"7:30-8 someproject
8-8:30 admin
- discussing staffing with colleague"#;

    let data = parse_time_tracking_data(input);
    let output = generate_sample_output(&data);

    assert!(output.contains("Start Time: 7:30 End Time: 8:30"));
    assert!(output.contains("Total Working Time: 1:00 (1.00 hrs)"));
    assert!(output.contains("Total dead time: 0:00 (0.00 hrs)"));
    assert!(output.contains("Billing Code: admin - 0:30 (0.50 hrs)"));
    assert!(output.contains("Billing Code: someproject - 0:30 (0.50 hrs)"));
    assert!(output.contains("- discussing staffing with colleague"));
}

#[test]
fn test_parse_large_gap_dead_time() {
    let input = r#"11:45-12:15 code1
- Comment explaining what you did
12:15-1:30 code2
- Comment about what you were doing
1:30-2 code1
2-4 code3
3:45-4 code4"#;

    let data = parse_time_tracking_data(input);
    
    println!("Debug: Total minutes: {}", data.total_minutes);
    println!("Debug: Dead time minutes: {}", data.dead_time_minutes);
    println!("Debug: Warnings: {:?}", data.warnings);

    // Total working time should be: 30 + 75 + 30 + 120 + 15 = 270 minutes (4.5 hours)
    assert_eq!(data.total_minutes, 270);
    
    // There should be a large gap from 4:00 to 3:45 (11 hours 45 minutes = 705 minutes)
    // This should both generate a warning AND be counted as dead time
    assert!(!data.warnings.is_empty());
    assert!(data.warnings.iter().any(|w| w.contains("Gap from 4:00 to 3:45")));
    
    // The dead time should include the large gap: 705 minutes (11:45)
    assert_eq!(data.dead_time_minutes, 705);
    
    // Check projects
    assert_eq!(data.projects.len(), 4);
    
    let code1 = data.projects.iter().find(|p| p.name == "code1").unwrap();
    assert_eq!(code1.total_minutes, 60); // 30 + 30 = 60 minutes
}

#[test]
fn test_parse_with_header_and_footer_content() {
    let input = r#"This is some header content
That should be ignored
Until we find time tracking data

11:45-12:15 code1
- Comment explaining what you did
12:15-1:30 code2
- Comment about what you were doing
1:30-2 code1
2-4 code3

This content after should be ignored
Because it doesn't start with a number, dash, or space
More content here
"#;

    let data = parse_time_tracking_data(input);
    
    // Should only parse the time tracking portion
    assert_eq!(data.total_minutes, 255); // 30 + 75 + 30 + 120 = 255 minutes
    assert_eq!(data.projects.len(), 3);
    
    let code1 = data.projects.iter().find(|p| p.name == "code1").unwrap();
    assert_eq!(code1.total_minutes, 60); // 30 + 30 = 60 minutes
    
    let code2 = data.projects.iter().find(|p| p.name == "code2").unwrap();
    assert_eq!(code2.total_minutes, 75); // 75 minutes
    
    let code3 = data.projects.iter().find(|p| p.name == "code3").unwrap();
    assert_eq!(code3.total_minutes, 120); // 120 minutes
}

#[test]
fn test_parse_stops_at_non_matching_line() {
    let input = r#"10-11 project1
- Note for project1
11-12 project2
Some random text that doesn't match pattern
1-2 project3
- This should not be parsed"#;

    let data = parse_time_tracking_data(input);
    
    // Should only parse the first two entries before hitting the non-matching line
    assert_eq!(data.total_minutes, 120); // 60 + 60 = 120 minutes
    assert_eq!(data.projects.len(), 2);
    
    // project3 should not be included
    assert!(!data.projects.iter().any(|p| p.name == "project3"));
}
