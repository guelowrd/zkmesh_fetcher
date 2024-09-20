use crate::errors::AppError;
use std::io;
use std::str::FromStr;
use chrono::NaiveDate;
use crate::config::read_blogs_from_file;
use tempfile::NamedTempFile;
use std::fs::File;
use std::io::Write;

#[test]
fn test_app_error_display() {

    //TODO: AppError::NetworkError

    let parse_error = AppError::ParseError("Invalid format".to_string());
    assert_eq!(parse_error.to_string(), "Parse error: Invalid format");

    let chrono_error = NaiveDate::from_str("invalid_date").unwrap_err();
    let date_parse_error = AppError::DateParseError(chrono_error);
    assert_eq!(date_parse_error.to_string(), "Date parse error: input contains invalid characters");

    let io_error = AppError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
    assert_eq!(io_error.to_string(), "IO error: File not found");

    let unknown_feed_type = AppError::UnknownFeedType("InvalidFeed".to_string());
    assert_eq!(unknown_feed_type.to_string(), "Unknown feed type: InvalidFeed");
}

#[test]
fn test_app_error_from_chrono_parse_error() {
    let chrono_error = NaiveDate::from_str("invalid_date").unwrap_err();
    let app_error: AppError = chrono_error.into();
    assert!(matches!(app_error, AppError::DateParseError(_)));
}

#[test]
fn test_app_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let app_error: AppError = io_error.into();
    assert!(matches!(app_error, AppError::IoError(_)));
}

#[test]
fn test_app_error_from_serde_json_error() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let app_error: AppError = AppError::ParseError(json_error.to_string());
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_app_error_from_atom_syndication_error() {
    let atom_error = atom_syndication::Error::InvalidStartTag;
    let app_error: AppError = AppError::ParseError(atom_error.to_string());
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_app_error_from_rss_error() {
    let rss_error = rss::Error::InvalidStartTag;
    let app_error: AppError = AppError::ParseError(rss_error.to_string());
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_app_error_from_serde_json() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let app_error: AppError = json_error.into();
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_app_error_from_atom_syndication() {
    let atom_error = atom_syndication::Error::InvalidStartTag;
    let app_error: AppError = atom_error.into();
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_app_error_from_rss() {
    let rss_error = rss::Error::InvalidStartTag;
    let app_error: AppError = rss_error.into();
    assert!(matches!(app_error, AppError::ParseError(_)));
}

#[test]
fn test_read_blogs_from_file_invalid_format() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();
    
    let mut file = File::create(path).unwrap();
    writeln!(file, "InvalidLine|MissingFeedType").unwrap();
    
    let result = read_blogs_from_file(path);
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, AppError::ParseError(_)));
        assert_eq!(
            e.to_string(),
            "Parse error: Invalid line format: InvalidLine|MissingFeedType"
        );
    }
}
