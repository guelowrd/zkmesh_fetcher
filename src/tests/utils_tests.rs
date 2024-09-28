use crate::utils::{parse_rss_date, capitalize_title, write_output};
use chrono::NaiveDate;
use std::fs::File;

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
    );
    assert_eq!(
        parse_rss_date(date3).expect("Failed to parse date3"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    
    assert!(parse_rss_date("Invalid Date").is_err());
}

#[test]
fn test_capitalize_title() {
    assert_eq!(capitalize_title("hello world"), "Hello World");
    assert_eq!(capitalize_title("a tale of two cities"), "A Tale of Two Cities");
    assert_eq!(capitalize_title("the quick brown fox"), "The Quick Brown Fox");
    assert_eq!(capitalize_title("and the rest"), "And the Rest");
    assert_eq!(capitalize_title("ZKSYNC is great"), "ZKsync Is Great");
    assert_eq!(capitalize_title("AGGLAYER’s new features"), "AggLayer’s New Features");
    assert_eq!(capitalize_title("Agglayer stuffs: this is (zUpEr) COOL"), "AggLayer Stuffs: This Is (zUpEr) COOL");
    assert_eq!(capitalize_title(""), ""); // Test empty string
}

#[test]
fn test_write_output() {
    let html_content = "<html><body><h1>Test</h1></body></html>";
    let result = write_output(html_content);
    assert!(result.is_ok());

    // Check if the file was created
    let output_file = File::open("./output/index.html");
    assert!(output_file.is_ok());
}