use crate::utils::parse_rss_date;
use chrono::NaiveDate;

#[test]
fn test_parse_rss_date() {
    let date1 = "Tue, 01 Oct 2024 12:00:00 GMT";
    let date2 = "2024-10-01T12:00:00+00:00";
    let date3 = "2024-10-01";
    
    assert_eq!(
        parse_rss_date(date1).expect("Failed to parse date1"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    assert_eq!(
        parse_rss_date(date2).expect("Failed to parse date2"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );assert_eq!(
        parse_rss_date(date3).expect("Failed to parse date3"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    
    assert!(parse_rss_date("Invalid Date").is_err());
}