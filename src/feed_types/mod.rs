mod substack;
mod rss;
mod atom;

pub use substack::SubstackFetcher;
pub use rss::RSSFetcher;
pub use atom::AtomFetcher;

use chrono::NaiveDate;
use crate::BlogArticle;

pub trait ArticleFetcher {
    fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedType {
    Substack,
    RSS,
    Atom,
}

impl std::str::FromStr for FeedType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Substack" => Ok(FeedType::Substack),
            "RSS" => Ok(FeedType::RSS),
            "Atom" => Ok(FeedType::Atom),
            _ => Err(format!("Unknown feed type: {}", s)),
        }
    }
}