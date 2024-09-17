#[cfg(test)]
mod tests;

use chrono::NaiveDate;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod feed_types;
use feed_types::{FeedType, ArticleFetcher, SubstackFetcher, RSSFetcher, AtomFetcher};

#[derive(Debug)]
pub struct BlogInfo {
    pub name: String,
    pub domain: String,
    pub feed_type: FeedType,
}

#[derive(Debug)]
pub struct BlogArticle {
    pub title: String,
    pub url: String,
    pub date: NaiveDate,
    pub blog_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let blogs = read_blogs_from_file("blogs.txt")?;
    let since_date = NaiveDate::parse_from_str("2024-09-01", "%Y-%m-%d")?;

    for blog in blogs {
        let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
            FeedType::Substack => Box::new(SubstackFetcher),
            FeedType::RSS => Box::new(RSSFetcher),
            FeedType::Atom => Box::new(AtomFetcher),
        };

        let articles = fetcher.fetch_articles(&blog.domain, &since_date, &blog.name)?;

        for article in articles {
            println!("[{}]({}) | {} ({})", article.title, article.url, article.blog_name, article.date);
        }
    }

    Ok(())
}

pub fn read_blogs_from_file(filename: &str) -> Result<Vec<BlogInfo>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let blogs: Vec<BlogInfo> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                Some(BlogInfo {
                    name: parts[0].trim().to_string(),
                    domain: parts[1].trim().to_string(),
                    feed_type: parts[2].trim().parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect();
    Ok(blogs)
}

pub fn parse_rss_date(date_str: &str) -> Result<NaiveDate, Box<dyn Error>> {
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

    Err(format!("Unable to parse date: {}", date_str).into())
}