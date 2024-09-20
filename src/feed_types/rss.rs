use super::ArticleFetcher;
use crate::BlogArticle;
use crate::errors::AppError;
use crate::utils::parse_rss_date;
use chrono::NaiveDate;
use reqwest::blocking::Client;
use rss::Channel;

pub struct RssFetcher;

impl ArticleFetcher for RssFetcher {
    fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        let client = Client::new();
        let response = client.get(feed_url).send()?;
        let content = response.bytes()?;
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