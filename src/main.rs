use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rss::Channel;

struct BlogInfo {
    name: String,
    domain: String,
    platform: String,
}

struct BlogArticle {
    title: String,
    url: String,
    date: NaiveDate,
    blog_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let blogs = read_blogs_from_file("blogs.txt")?;
    let since_date = NaiveDate::parse_from_str("2024-09-01", "%Y-%m-%d")?;

    for blog in blogs {
        let articles = match blog.platform.as_str() {
            "Substack" => {
                let api_url = format!("https://{}/api/v1/posts/?limit=50", blog.domain);
                fetch_substack_blog_articles(&api_url, &since_date, &blog.name, &blog.domain)?
            },
            "Medium" => {
                let feed_url = format!("https://medium.com/feed/{}", blog.domain.trim_start_matches("medium.com/"));
                fetch_rss_blog_articles(&feed_url, &since_date, &blog.name)?
            },
            "RSS" => {
                fetch_rss_blog_articles(&blog.domain, &since_date, &blog.name)?
            },
            _ => continue,
        };

        for article in articles {
            println!("[{}]({}) | {} ({})", article.title, article.url, article.blog_name, article.date);
        }
    }

    Ok(())
}

fn read_blogs_from_file(filename: &str) -> Result<Vec<BlogInfo>, Box<dyn Error>> {
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
                    platform: parts[2].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    Ok(blogs)
}

fn fetch_substack_blog_articles(api_url: &str, since_date: &NaiveDate, blog_name: &str, blog_domain: &str) -> Result<Vec<BlogArticle>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(api_url).send()?;
    let json: Value = response.json()?;

    let mut articles = Vec::new();

    if let Some(posts) = json.as_array() {
        for post in posts {
            let title = post["title"].as_str().unwrap_or_default().to_string();
            let slug = post["slug"].as_str().unwrap_or_default();
            let url = format!("https://{}/p/{}", blog_domain, slug);
            let date_str = post["post_date"].as_str().unwrap_or_default();
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?;

            if date >= *since_date {
                articles.push(BlogArticle { title, url, date, blog_name: blog_name.to_string() });
            }
        }
    }

    Ok(articles)
}

fn fetch_rss_blog_articles(feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(feed_url).send()?;
    let content = response.bytes()?;
    let channel = Channel::read_from(&content[..])?;

    let mut articles = Vec::new();

    for item in channel.items() {
        if let (Some(title), Some(link), Some(pub_date)) = (item.title(), item.link(), item.pub_date()) {
            let date = parse_rss_date(pub_date)?;
            if date >= *since_date {
                articles.push(BlogArticle {
                    title: title.to_string(),
                    url: link.to_string(),
                    date,
                    blog_name: blog_name.to_string(),
                });
            }
        }
    }

    Ok(articles)
}

fn parse_rss_date(date_str: &str) -> Result<NaiveDate, Box<dyn Error>> {
    let formats = [
        "%a, %d %b %Y %H:%M:%S %Z",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%d",
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    Err(format!("Unable to parse date: {}", date_str).into())
}