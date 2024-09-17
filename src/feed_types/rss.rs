use super::ArticleFetcher;
use crate::BlogArticle;
use chrono::NaiveDate;
use reqwest::blocking::Client;
use rss::Channel;

pub struct RSSFetcher;

impl ArticleFetcher for RSSFetcher {
    fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client.get(feed_url).send()?;
        let content = response.bytes()?;
        let channel = Channel::read_from(&content[..])?;

        let mut articles = Vec::new();

        for item in channel.items() {
            if let (Some(title), Some(link), Some(pub_date)) = (item.title(), item.link(), item.pub_date()) {
                let date = crate::parse_rss_date(pub_date)?;
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
}