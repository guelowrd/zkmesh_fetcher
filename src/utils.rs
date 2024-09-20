use chrono::NaiveDate;
use crate::errors::AppError;

pub fn parse_rss_date(date_str: &str) -> Result<NaiveDate, AppError> {
    let formats = [
        "%a, %d %b %Y %H:%M:%S %Z",
        "%a, %d %b %Y %H:%M:%S GMT",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%d",
        "%Y-%m-%dT%H:%M:%SZ",  
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    Err(AppError::ParseError(format!("Unable to parse date: {}", date_str)))
}