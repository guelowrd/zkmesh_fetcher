use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use serde_json::Value;
use async_trait::async_trait;

pub struct SubstackFetcher;

#[async_trait]
impl ArticleFetcher for SubstackFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        let client = reqwest::Client::new();
        let response = client.get(feed_url).send().await?;
        let json: Value = response.json().await?;

        let mut articles = Vec::new();

        if let Some(posts) = json.as_array() {
            for post in posts {
                let title = post["title"].as_str().ok_or_else(|| AppError::ParseError("Missing title".to_string()))?.to_string();
                let slug = post["slug"].as_str().ok_or_else(|| AppError::ParseError("Missing slug".to_string()))?;
                let url = format!("{}/p/{}", feed_url.trim_end_matches("/api/v1/posts/?limit=50"), slug);
                let date_str = post["post_date"].as_str().ok_or_else(|| AppError::ParseError("Missing post_date".to_string()))?;
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?;
                if date >= *since_date {
                    articles.push(BlogArticle {
                        title,
                        url,
                        date,
                        blog_name: blog_name.to_string(),
                        authors: None, // Set authors to None for Substack
                    });
                }
            }
        }

        Ok(articles)
    }
}