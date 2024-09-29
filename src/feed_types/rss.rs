use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use crate::utils::parse_rss_date;
use chrono::NaiveDate;
use rss::Channel;
use async_trait::async_trait;

pub struct RssFetcher;

#[async_trait]
impl ArticleFetcher for RssFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str, custom_url_replace: Option<String>) -> Result<Vec<BlogArticle>, AppError> {
        let content = reqwest::get(feed_url).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut articles = Vec::new();

        for item in channel.items() {
            let title = item.title().ok_or_else(|| AppError::ParseError("Missing title".to_string()))?;
            let link = item.link().ok_or_else(|| AppError::ParseError("Missing link".to_string()))?;
            let pub_date = item.pub_date().ok_or_else(|| AppError::ParseError("Missing publication date".to_string()))?;

            let date = parse_rss_date(pub_date)?;

            // Handle custom URL replacement
            let url = if let Some(replace) = &custom_url_replace {
                let parts: Vec<&str> = replace.split('>').collect();
                if parts.len() == 2 {
                    let old_url = parts[0].trim();
                    let new_url = parts[1].trim();
                    link.replace(old_url, new_url) // Replace the erroneous URL part
                } else {
                    link.to_string() // Convert to String if the format is incorrect
                }
            } else {
                link.to_string() // Convert to String if no replacement is specified
            };

            if date >= *since_date {
                articles.push(BlogArticle {
                    title: title.to_string(),
                    url,
                    date,
                    blog_name: blog_name.to_string(),
                    authors: None, // Set authors to None for RSS
                });
            }
        }

        Ok(articles)
    }
}