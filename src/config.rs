use std::fs::File;
use std::io::BufReader;
use crate::errors::AppError;
use crate::models::BlogInfo;
use serde_json;

pub fn read_blogs_from_file(filename: &str) -> Result<Vec<BlogInfo>, AppError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let blogs: Vec<BlogInfo> = serde_json::from_reader(reader)
        .map_err(|e| AppError::ParseError(format!("Failed to parse JSON: {}", e)))?;
    Ok(blogs)
}