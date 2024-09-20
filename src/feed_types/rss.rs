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
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        let content = reqwest::get(feed_url).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut articles = Vec::new();

        for item in channel.items() {
            let title = item.title().ok_or_else(|| AppError::ParseError("Missing title".to_string()))?;
            let link = item.link().ok_or_else(|| AppError::ParseError("Missing link".to_string()))?;
            let pub_date = item.pub_date().ok_or_else(|| AppError::ParseError("Missing publication date".to_string()))?;
            
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

        Ok(articles)
    }
}