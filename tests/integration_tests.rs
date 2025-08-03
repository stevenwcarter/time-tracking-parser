use time_tracking_parser::*;

#[test]
fn test_integration_sample_data() {
    let input = r#"7:30-8 someproject
8-8:30 general
- discussing staffing with colleague
8:30-11 someproject
- investigated issue, pushed PR
- pushed potential fix for component /build url handling
11-12 other-project
- tech connect
12-12:30 general
- 1:1 w/ coworker
12:30-2:30 someproject
- discussing work items and how to complete"#;

    let data = parse_time_tracking_data(input);
    let output = generate_sample_output(&data);

    // Verify the output matches expected format
    let expected_lines = [
        "Start Time: 7:30 End Time: 2:30",
        "Total Working Time: 7:00 (7.00 hrs)",
        "Total dead time: 0:00 (0.00 hrs)",
    ];

    for expected_line in expected_lines {
        assert!(
            output.contains(expected_line),
            "Output missing expected line: {expected_line}\nActual output:\n{output}"
        );
    }

    // Verify project sections exist
    assert!(output.contains("Billing Code: someproject - 5:00 (5.00 hrs)"));
    assert!(output.contains("Billing Code: general - 1:00 (1.00 hrs)"));
    assert!(output.contains("Billing Code: other-project - 1:00 (1.00 hrs)"));

    // Verify specific notes are included
    assert!(
        output.contains("- investigating issue, pushed PR")
            || output.contains("- investigated issue, pushed PR")
    );
    assert!(output.contains("- discussing staffing with colleague"));
    assert!(output.contains("- tech connect"));
    assert!(output.contains("- 1:1 w/ coworker"));
}

#[test]
fn test_wasm_compatibility() {
    // Test that the main parsing function works (this ensures WASM compatibility)
    let input = "7-8 test\n- note";
    let result = parse_time_data(input);

    assert!(result.contains("Start Time: 7:00 End Time: 8:00"));
    assert!(result.contains("Billing Code: test - 1:00 (1.00 hrs)"));
    assert!(result.contains("- note"));
}

#[test]
fn test_edge_case_midnight_crossing() {
    let input = r#"11:30-12:30 project1
12:30-1:30 project2"#;

    let data = parse_time_tracking_data(input);

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

    let data = parse_time_tracking_data(input);

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
    assert_eq!(data.projects.len(), 3);

    // Check project1 aggregation
    let project1 = data.projects.iter().find(|p| p.name == "project1").unwrap();
    assert_eq!(project1.total_minutes, 180); // 1 + 2 hours = 180 minutes
    assert_eq!(project1.notes.len(), 2);
}

#[test]
fn test_twelve_hour_time_boundaries() {
    let input = r#"12-1 project1
1-2 project2
11-12 project3"#;

    let data = parse_time_tracking_data(input);

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

    let data = parse_time_tracking_data(&input);

    // Should handle large inputs without issues
    assert_eq!(data.projects.len(), 5); // 5 unique projects (0-4)
    assert_eq!(data.total_minutes, 100 * 60); // 100 hours
}
