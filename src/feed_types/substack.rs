use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use async_trait::async_trait;
use crate::utils::replace_url;

pub struct SubstackFetcher;

#[async_trait]
impl ArticleFetcher for SubstackFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str, custom_url_replace: Option<String>) -> Result<Vec<BlogArticle>, AppError> {
        let response = reqwest::get(feed_url).await?.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response)?;

        let mut articles = Vec::new();

        if let Some(posts) = json.as_array() {
            for post in posts {
                let title = post["title"].as_str().ok_or_else(|| AppError::ParseError("Missing title".to_string()))?.to_string();
                let slug = post["slug"].as_str().ok_or_else(|| AppError::ParseError("Missing slug".to_string()))?;
                let url = format!("{}/p/{}", feed_url.trim_end_matches("/api/v1/posts/?limit=50"), slug);
                let date_str = post["post_date"].as_str().ok_or_else(|| AppError::ParseError("Missing post_date".to_string()))?;
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?;

                // Handle custom URL replacement
                let final_url = replace_url(&url, custom_url_replace.as_ref());

                if date >= *since_date {
                    articles.push(BlogArticle {
                        title,
                        url: final_url,
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