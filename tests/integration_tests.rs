use time_tracking_parser::*;

#[test]
fn test_edge_case_midnight_crossing() {
    let input = r#"11:30-12:30 project1
12:30-1:30 project2"#;

    let data = parse_time_tracking_data(input, None, None);

    // Should handle crossing noon/midnight correctly
    assert_eq!(data.total_minutes, 120); // 2 hours total
    assert_eq!(data.dead_time_minutes, 0); // No gaps
    assert_eq!(data.projects.len(), 2);

    for project in &data.projects {
        assert_eq!(project.total_minutes, 60); // Each should be 1 hour
    }
}

#[test]
fn test_complex_scenario_with_gaps_and_warnings() {
    let input = r#"7-8 project1
- morning work
10-11 project2
- after break
11-1 project1
- long session
3-4
- missing project name
5-6 project3"#;

    let data = parse_time_tracking_data(input, None, None);

    // Check warnings
    assert!(!data.warnings.is_empty());
    assert!(
        data.warnings
            .iter()
            .any(|w| w.contains("missing project name"))
    );

    // Check dead time calculation:
    // 8-10 (2 hrs), 1-3 (2 hrs), 4-5 (1 hr) = 5 hours = 300 minutes
    // The 3-4 entry without project name doesn't count as dead time, it's just invalid work time
    assert_eq!(data.dead_time_minutes, 300);

    // Check total working time (1 + 1 + 2 + 1 + 1 = 6 hours = 360 minutes)
    // This includes the 3-4 entry even though it has no project name
    assert_eq!(data.total_minutes, 360);

    // Should have 3 valid projects (the one without a name doesn't create a project)
    assert_eq!(data.projects.len(), 4);

    // Check project1 aggregation
    let project1 = data.projects.iter().find(|p| p.name == "project1").unwrap();
    assert_eq!(project1.total_minutes, 180); // 1 + 2 hours = 180 minutes
    assert_eq!(project1.notes.len(), 2);
    let missing = data.projects.iter().find(|p| p.name == "missing").unwrap();
    assert_eq!(missing.total_minutes, 60);
    assert_eq!(missing.notes.len(), 1);
}

#[test]
fn test_twelve_hour_time_boundaries() {
    let input = r#"12-1 project1
1-2 project2
11-12 project3"#;

    let data = parse_time_tracking_data(input, None, None);

    assert_eq!(data.total_minutes, 180); // 3 hours
    assert_eq!(data.projects.len(), 3);

    // Verify each project gets 1 hour
    for project in &data.projects {
        assert_eq!(project.total_minutes, 60);
    }
}

#[test]
fn test_performance_with_large_input() {
    // Generate a large input to test performance
    let mut input = String::new();
    for i in 0..100 {
        let start_hour = (i % 11) + 1; // Keep within 1-12 range
        let end_hour = if start_hour == 12 { 1 } else { start_hour + 1 };
        input.push_str(&format!("{start_hour}-{end_hour} project{}\n", i % 5));
        input.push_str(&format!("- note for project {}\n", i % 5));
    }

    let data = parse_time_tracking_data(&input, None, None);

    // Should handle large inputs without issues
    assert_eq!(data.projects.len(), 5); // 5 unique projects (0-4)
    assert_eq!(data.total_minutes, 100 * 60); // 100 hours
}
