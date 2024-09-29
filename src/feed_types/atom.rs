use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use crate::utils::parse_rss_date;
use chrono::NaiveDate;
use atom_syndication::Feed;
use async_trait::async_trait;
use crate::utils::replace_url;

pub struct AtomFetcher;

#[async_trait]
impl ArticleFetcher for AtomFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str, custom_url_replace: Option<String>) -> Result<Vec<BlogArticle>, AppError> {
        let content = reqwest::get(feed_url).await?.bytes().await?;
        let feed = Feed::read_from(&content[..])?;

        let mut articles = Vec::new();

        for entry in feed.entries() {
            let title = entry.title().to_string();
            let link = entry.links.first()
                .ok_or_else(|| AppError::ParseError("Missing link".to_string()))?
                .href.clone();
            let date = parse_rss_date(&entry.updated.to_rfc2822())?;

            // Handle custom URL replacement
            let url = replace_url(&link, custom_url_replace.as_ref());

            if date >= *since_date {
                articles.push(BlogArticle {
                    title,
                    url,
                    date,
                    blog_name: blog_name.to_string(),
                    authors: None, // Set authors to None for Atom
                });
            }
        }

        Ok(articles)
    }
}