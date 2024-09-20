use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::errors::AppError;
use crate::models::BlogInfo;

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