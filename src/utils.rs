use chrono::NaiveDate;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::errors::AppError;
use crate::BlogInfo;

pub fn read_blogs_from_file(filename: &str) -> Result<Vec<BlogInfo>, AppError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut blogs = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let feed_type = parts[2].trim().parse()?;
            blogs.push(BlogInfo {
                name: parts[0].trim().to_string(),
                domain: parts[1].trim().to_string(),
                feed_type,
            });
        } else {
            return Err(AppError::ParseError(format!("Invalid line format: {}", line)));
        }
    }
    Ok(blogs)
}

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