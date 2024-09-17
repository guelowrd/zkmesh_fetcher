use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct BlogInfo {
    name: String,
    domain: String,
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
        let api_url = format!("https://{}/api/v1/posts/?limit=50", blog.domain);
        let articles = fetch_substack_blog_articles(&api_url, &since_date, &blog.name, &blog.domain)?;

        for article in articles {
            println!("[{}]({}) ({}) | {}", article.title, article.url, article.date, article.blog_name);
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
            if parts.len() == 2 {
                Some(BlogInfo {
                    name: parts[0].trim().to_string(),
                    domain: parts[1].trim().to_string(),
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