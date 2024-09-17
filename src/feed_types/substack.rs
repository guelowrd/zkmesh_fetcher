use super::ArticleFetcher;
use crate::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde_json::Value;

pub struct SubstackFetcher;

impl ArticleFetcher for SubstackFetcher {
    fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        let client = Client::new();
        let response = client.get(feed_url).send()?;
        let json: Value = response.json()?;

        let mut articles = Vec::new();

        if let Some(posts) = json.as_array() {
            for post in posts {
                let title = post["title"].as_str().unwrap_or_default().to_string();
                let slug = post["slug"].as_str().unwrap_or_default();
                let url = format!("{}/p/{}", feed_url.trim_end_matches("/api/v1/posts/?limit=50"), slug);
                let date_str = post["post_date"].as_str().unwrap_or_default();
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?;
                if date >= *since_date {
                    articles.push(BlogArticle { title, url, date, blog_name: blog_name.to_string() });
                }
            }
        }

        Ok(articles)
    }
}