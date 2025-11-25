use time_tracking_parser::*;

#[test]
fn test_time_creation_valid() {
    let time = Time::new(7, 30).unwrap();
    assert_eq!(time.hour, 7);
    assert_eq!(time.minute, 30);
}

#[test]
fn test_time_creation_hour_boundary() {
    assert!(Time::new(1, 0).is_ok());
    assert!(Time::new(12, 0).is_ok());
    assert!(Time::new(0, 0).is_err());
    assert!(Time::new(13, 0).is_err());
}

#[test]
fn test_time_creation_minute_boundary() {
    assert!(Time::new(7, 0).is_ok());
    assert!(Time::new(7, 59).is_ok());
    assert!(Time::new(7, 60).is_err());
}

#[test]
fn test_time_to_minutes() {
    let time1 = Time::new(1, 0).unwrap();
    assert_eq!(time1.to_minutes(), 60);

    let time2 = Time::new(12, 0).unwrap();
    assert_eq!(time2.to_minutes(), 0); // 12 AM = 0 minutes

    let time3 = Time::new(7, 30).unwrap();
    assert_eq!(time3.to_minutes(), 450); // 7:30 AM = 7.5 * 60 = 450
}

#[test]
fn test_duration_minutes_same_am_pm() {
    let start = Time::new(7, 30).unwrap();
    let end = Time::new(8, 0).unwrap();
    assert_eq!(start.duration_minutes(&end), 30);
}

#[test]
fn test_duration_minutes_cross_noon() {
    let start = Time::new(11, 30).unwrap();
    let end = Time::new(1, 0).unwrap(); // 1 PM 
    assert_eq!(start.duration_minutes(&end), 90); // 11:30 AM to 1:00 PM = 1.5 hours
}

#[test]
fn test_duration_minutes_same_time() {
    let start = Time::new(7, 30).unwrap();
    let end = Time::new(7, 30).unwrap();
    assert_eq!(start.duration_minutes(&end), 0);
}

#[test]
fn test_format_duration_minutes() {
    assert_eq!(Time::format_duration_minutes(0), "0:00");
    assert_eq!(Time::format_duration_minutes(60), "1:00");
    assert_eq!(Time::format_duration_minutes(90), "1:30");
    assert_eq!(Time::format_duration_minutes(420), "7:00");
    assert_eq!(Time::format_duration_minutes(450), "7:30");
}

#[test]
fn test_format_duration_decimal() {
    assert_eq!(Time::format_duration_decimal(0), "0.00");
    assert_eq!(Time::format_duration_decimal(60), "1.00");
    assert_eq!(Time::format_duration_decimal(90), "1.50");
    assert_eq!(Time::format_duration_decimal(420), "7.00");
    assert_eq!(Time::format_duration_decimal(450), "7.50");
}

#[test]
fn test_hour() {
    assert_eq!("1".parse::<Hour>().unwrap(), 1);
    assert_eq!("0".parse::<Hour>().unwrap(), 0);
    assert_eq!("12".parse::<Hour>().unwrap(), 12);
    assert!("13".parse::<Hour>().is_err());
    assert!("-3".parse::<Hour>().is_err());
}
